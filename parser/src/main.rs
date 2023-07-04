
// data available here: https://database.lichess.org/


use std::process::Command;
use pgn_reader::{Visitor, Skip, BufferedReader, SanPlus};


mod disambiguation;
mod filenameiterator;
mod pgniterator;

use filenameiterator::FileNameIterator;
use pgniterator::pgn_records;

fn main() {
    let data_directory = "../data";
    unzip_all(data_directory);
    let filenames = FileNameIterator::new(data_directory).has_extension("pgn");
    for filename in filenames {
        let pgn_records = pgn_records(filename.as_str()).unwrap();
        
        
        //println!("{:?}", pgn_records.next());
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

