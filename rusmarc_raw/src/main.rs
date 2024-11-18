use std::io::Write;
use std::{fs::File, io::BufReader};

use rusmarc_raw::record::{record_remove_errors, RecordsReader};

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

    let mut i = 0usize;
    for record in parser {
        let record = record_remove_errors(record.unwrap());
        i += 1;

        writeln!(
            &mut writer,
            "{i} => {}",
            serde_json::to_string_pretty(&record).unwrap()
        )
        .unwrap();

        if i % 1000 == 0 {
            writer.flush().unwrap();
            println!("Entry {i}");
        }
    }
}
