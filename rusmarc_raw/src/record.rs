use std::io::{BufRead, BufReader, Cursor};

use crate::field::*;

pub type Record = Vec<Result<Field, crate::field::Error>>;
pub type ErrorlessRecord = Vec<Field>;

pub fn record_remove_errors(record: Record) -> ErrorlessRecord {
    record
        .into_iter()
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect()
}

pub struct RecordsReader<T: BufRead> {
    input: T,
}

impl<T: BufRead> RecordsReader<T> {
    pub fn new(input: T) -> Self {
        Self { input }
    }
}

impl<T: BufRead> Iterator for RecordsReader<T> {
    type Item = Result<Record, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut total_text = String::new();

        // Collect entire entry text into string
        'mainloop: loop {
            let bytes_read;

            match self.input.read_line(&mut total_text) {
                Ok(bytes_read_in) => {
                    bytes_read = bytes_read_in;
                    if bytes_read == 0 {
                        break 'mainloop; // EOF
                    }
                }
                Err(err) => return Some(Err(err)),
            }

            let last_line = &total_text[(total_text.len() - bytes_read)..];
            if does_string_only_has_a_char(last_line.trim(), '*') {
                break;
            }
        }

        if total_text.len() == 0 {
            return None;
        }

        // Parse each field
        let cursor = Cursor::new(total_text);
        let buf_reader = BufReader::new(cursor);
        let parser = FieldsReader::new(buf_reader);

        let record: Record = parser.collect();
        return Some(Ok(record));
    }
}

fn does_string_only_has_a_char(text: &str, c: char) -> bool {
    for text_char in text.chars() {
        if text_char != c {
            return false;
        }
    }

    if text.len() == 0 {
        return false;
    }

    return true;
}
