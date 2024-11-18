use std::io::{self, BufRead};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Subfield {
    pub marker: char,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FieldData {
    FullLine { text: String },
    Subfields { subfields: Vec<Subfield> },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    pub number: u128,
    pub data: FieldData,
}

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    NoNumberPresent,
}

#[derive(Debug)]
pub enum FieldReadResult {
    Ok(Field),
    Err(crate::field::Error),
    Empty,
    End,
}

#[derive(Debug)]
pub struct FieldsReader<T: BufRead> {
    input: T,
}

impl<T: io::BufRead> FieldsReader<T> {
    pub fn new(input: T) -> Self {
        return Self { input };
    }

    pub fn read(&mut self) -> FieldReadResult {
        let mut line = String::new();
        let result = self.input.read_line(&mut line);

        // Check for IO error or EOF
        match result {
            Err(err) => return FieldReadResult::Err(Error::IoError(err)),
            Ok(bytes_read) => {
                if bytes_read == 0 && line.len() == 0 {
                    return FieldReadResult::End; // EOF
                }
            }
        };

        // Remove LF or CRLF from the end
        if line.ends_with("\n") {
            line.remove(line.len() - 1);
        }
        if line.ends_with("\r") {
            line.remove(line.len() - 1);
        }

        // Parse
        let line = line.trim();
        if line.len() == 0 {
            return FieldReadResult::Empty;
        }

        // Line starts are: `#<integer>:`
        // Due to format being undocumented, parser will allow omitting `#` and `:`
        let (number_part, data_part) = get_number_and_data_parts(&line);

        if number_part.len() == 0 {
            return FieldReadResult::Err(Error::NoNumberPresent);
        }
        if data_part.len() == 0 {
            return FieldReadResult::Empty;
        }

        // Parse the number
        let number: u128 = number_part.parse().unwrap();

        // Parse the data
        let data = if data_part.starts_with("^") {
            let subfields: Vec<_> = data_part
                .split("^")
                .into_iter()
                .filter(|x| x.len() > 1)
                .map(|x| {
                    let mut indices = x.char_indices();
                    let (_, first_char) = indices.next().unwrap();
                    let (offset, _) = indices.next().unwrap();
                    Subfield {
                        marker: first_char,
                        text: x[offset..].to_owned(),
                    }
                })
                .collect();

            FieldData::Subfields { subfields }
        } else {
            FieldData::FullLine {
                text: data_part.to_string(),
            }
        };

        FieldReadResult::Ok(Field { number, data })
    }
}

fn get_number_and_data_parts(mut line: &str) -> (&str, &str) {
    // 1. Find where number starts
    if line.starts_with("#") {
        // skip hashtag
        line = &line[1..];
        line = line.trim_start();
    }

    // 2. Find amount of characters in the number
    let mut number_end = 0;
    for (i, c) in line.char_indices() {
        if !c.is_digit(10) {
            number_end = i;
            break;
        }
    }
    let number_end = number_end;
    let number_part = &line[..number_end];
    line = &line[number_end..];

    // 3. Trim and skip possible colon (`:`)
    line = line.trim_start();
    if line.starts_with(":") {
        line = &line[1..];
        line = line.trim_start();
    }

    // 4. Whatever is left is the data
    let data_part = line;

    (number_part, data_part)
}

impl<T: io::BufRead> Iterator for FieldsReader<T> {
    type Item = Result<Field, crate::field::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.read() {
                FieldReadResult::Ok(field) => return Some(Ok(field)),
                FieldReadResult::Err(err) => return Some(Err(err)),
                FieldReadResult::End => return None,
                FieldReadResult::Empty => continue,
            }
        }
    }
}
