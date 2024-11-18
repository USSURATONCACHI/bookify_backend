use std::io::Write;
use std::{fs::File, io::BufReader};

use rusmarc_raw::record::{record_remove_errors, RecordsReader};
use rusmarc_raw::typed_record::{Field001RecordId, TypedRecord};

fn main() {
    println!("Hello, world!");

    let stderr = std::io::stderr();
    let handle = stderr.lock(); // Get a locked handle to stdout
    let mut writer = std::io::BufWriter::new(handle);

    writeln!(&mut writer, "Number: {}", 123).unwrap();

    let filepath = "/home/ussur/Desktop/lan_zipssga_zipznanium_zip/LAN.TXT";
    let file = File::open(filepath).unwrap();

    let buf_reader = BufReader::new(file);
    let parser = RecordsReader::new(buf_reader);

    for (i, record) in parser.into_iter().enumerate() {
        let record = record_remove_errors(record.unwrap());

        let (record, errors) = TypedRecord::parse(record.into_iter());
        writeln!(&mut writer, "Errors count: {:?}", errors.len()).unwrap();

        for f001 in record.get_fields::<Field001RecordId>() {
            writeln!(&mut writer, "Field 001: {:?}", f001).unwrap();
        }

        if i % 1000 == 0 {
            writer.flush().unwrap();
        }
    }
}
