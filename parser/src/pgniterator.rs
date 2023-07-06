use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::marker::PhantomData;

use crate::disambiguation::{parse};


#[derive(Debug)]
pub struct Record<T> {
    data: T,
    white_elo: u16,
    black_elo: u16,
    result : u16, // 0 black wins, 1 draw, 2 white wins
}

pub fn write_binary_record_to_file(record: &BinaryRecord, file: &mut File) -> io::Result<()> {
    // Write white_elo, black_elo, and result
    file.write_all(&record.result.to_le_bytes())?;
    file.write_all(&record.white_elo.to_le_bytes())?;
    file.write_all(&record.black_elo.to_le_bytes())?;
    
    // Write data vector length
    let data_len = record.data.len() as u16;
    file.write_all(&data_len.to_le_bytes())?;
    // Write data vector
    file.write_all(&record.data.iter().flat_map(|&x| x.to_le_bytes()).collect::<Vec<_>>())?;
   
    Ok(())
}

pub type BinaryRecord = Record<Vec<u16>>;
pub type StringRecord = Record<String>;

pub struct BinaryParser;
pub struct StringParser;

pub trait TNotationParse<T> {
    fn parse(&self, png_data : &str, white_elo: u16, black_elo: u16, result : u16) -> Record<T>;
}

impl TNotationParse<Vec<u16>> for BinaryParser {
    fn parse(&self, png_data : &str, white_elo: u16, black_elo: u16, result : u16) -> BinaryRecord {
        let data = parse(png_data, Vec::new()).unwrap();
        return BinaryRecord { data : data, white_elo : white_elo, black_elo : black_elo, result : result}
    }

}

impl TNotationParse<String> for StringParser {
    fn parse(&self, png_data : &str, white_elo: u16, black_elo: u16, result : u16) -> StringRecord {
       // let data = parse_to_string(png_data).unwrap();
        let data = parse(png_data, String::new()).unwrap();
        return StringRecord { data : data, white_elo : white_elo, black_elo : black_elo, result : result}
    }
}

pub struct RecordIterator<R, X, T : TNotationParse<X>> {
    reader: R,
    record_parser : T,
    _phantom : PhantomData<X>,
}

impl<R,X,T : TNotationParse<X>> RecordIterator<R, X, T> {
    
    fn get_result(input: &str) -> u16 {
        if input == "0-1" {
            return 0;
        } else if input == "1-0" {
            return 2;
        } else if input == "1/2-1/2" {
            return 1;
        }
        panic!("Unknown result: {}", input);
    }

    fn extract_field_value<'a>(line: &'a str, field: &'a str) -> Option<&'a str> {
        let field_start = line.find(field)?;
        let value_start = field_start + field.len() + 2;
        let value_end = line[value_start..].find('"')? + value_start;
        Some(&line[value_start..value_end])
    }
}


impl<R: BufRead, X, T : TNotationParse<X>> Iterator for RecordIterator<R, X, T> {
    type Item = Result<Record<X>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut pgn_data = String::new();
        let mut white_elo = None;
        let mut black_elo = None;
        let mut result = None;

        while let Ok(line) = self.reader.read_line(&mut pgn_data) {
            if line == 0 {
                break;
            }
            if let Some(field_value) = RecordIterator::<R,X,T>::extract_field_value(&pgn_data, "WhiteElo") {
                white_elo = field_value.parse::<u16>().ok();                
            } else if let Some(field_value) = RecordIterator::<R,X,T>::extract_field_value(&pgn_data, "BlackElo") {
                black_elo = field_value.parse::<u16>().ok();
            } else if let Some(field_value) = RecordIterator::<R,X,T>::extract_field_value(&pgn_data, "Result") {
                result = Some(RecordIterator::<R,X,T>::get_result(field_value));
            } else if pgn_data.starts_with("1.") {
                if white_elo.is_none() {
                    return Some(Err("white elo missing.".to_string()));
                } else if black_elo.is_none() {
                    return Some(Err("black elo missing.".to_string()));
                } else {
                    let x = self.record_parser.parse(&pgn_data, white_elo.unwrap(), black_elo.unwrap(), result.unwrap());
                    return Some(Ok(x));
                }
            }
            pgn_data.clear();
        }
        return None;
    }
}

pub fn records<X, T : TNotationParse<X>>(file_path: &str, record_parser : T) -> io::Result<RecordIterator<BufReader<File>, X, T>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let iter = RecordIterator { reader, record_parser, _phantom : PhantomData };
    return Ok(iter);
}






/* // OLD PARSING LOGIC
use lazy_static::lazy_static;

lazy_static! {
    static ref MOVE_NUMBER_PATTERN: Regex = Regex::new(r"\d+\.").unwrap();
    static ref EVAL_PATTERN : Regex = Regex::new(r"\{[^{}]*\}").unwrap();
    static ref SQUASH_SPACES : Regex = Regex::new(r"\s+").unwrap();
}

fn squash_spaces(text: &str) -> String {
    SQUASH_SPACES.replace_all(text, " ").into()
}

fn remove_move_numbers(input: &str) -> String {
    return MOVE_NUMBER_PATTERN.replace_all(input, "").replace("..", "").to_string()
}

fn remove_curly_brackets(input: &str) -> String {
    EVAL_PATTERN.replace_all(input, "").to_string()
}


fn get_last_n_chars(input: &str, n: usize) -> &str {
    let len = input.chars().count();
    if n >= len {
        return input;
    }
    let start_index = len - n;
    &input[start_index..]
}
*/