
// data available here: https://database.lichess.org/


use std::process::Command;

use std::fs::File;
use std::io::{self, Write};

use zzz::ProgressBarIterExt as _;

mod disambiguation;
mod filenameiterator;
mod pgniterator;

use filenameiterator::FileNameIterator;
use pgniterator::{records, write_binary_record_to_file,BinaryParser, StringParser, BinaryRecord};

fn main() {
    let data_directory = "../data";
    unzip_all(data_directory);
    let filenames = FileNameIterator::new(data_directory).has_extension("pgn");

    for filename in filenames {
        println!("{}", filename);
       
        let mut file = File::create("./test.tnot").unwrap();
        let records = records(filename.as_str(), BinaryParser).unwrap();
        for result in records.into_iter().progress() {
            match result {
                Ok(record) => { write_binary_record_to_file(&record, &mut file); },
                Err(msg) => {},
            }
        }
        //
        //println!();
        //println!("{:?}", pgn_records.next());
    }
}





fn unzip_all(directory : &str) {
    let filenames = FileNameIterator::new(directory).has_extension("zst");
    for filename in filenames {
        println!("{:?}", filename);
        unzip(filename.as_str());
    }
}

fn unzip(filename : &str) {
    // pzstd must be installed!
    let output = Command::new("pzstd")
        .arg("-df")
        .arg(filename)
        .output()
        .expect("Failed to execute command.");

    if output.status.success() {
        println!("Unzipping {} succeeded!", filename);
        //let unzipped_data = String::from_utf8(output.stdout).expect("Invalid UTF-8 data.");
    } else {
        eprintln!("Unzipping {} failed!\nError:\n{}", filename, String::from_utf8_lossy(&output.stderr));
    }
}

