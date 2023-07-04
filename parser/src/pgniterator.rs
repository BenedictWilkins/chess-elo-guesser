use std::fs::File;
use std::io::{self, BufRead, BufReader};
use regex::Regex;

use crate::disambiguation::to_tnotation;

#[derive(Debug)]
pub struct PgnRecord {
    tnotation: String,
    white_elo: u16,
    black_elo: u16,
    result : u8, // 0 black wins, 1 draw, 2 white wins
}

pub struct PgnRecordIterator<R> {
    reader: R,
}

/*
fn main() -> io::Result<()> {
    let pgn = b"1. e4 { [%eval 0.2] } 1... e6 { [%eval 0.13] } 2. Bc4 { [%eval -0.31] } 2... d5 { [%eval -0.28] } 3. exd5 { [%eval -0.37] } 3... exd5 { [%eval -0.31] } 4. Bb3 { [%eval -0.33] } 4... Nf6 { [%eval -0.35] } 5. d4 { [%eval -0.34] } 5... Be7 { [%eval 0.0] } 6. Nf3 { [%eval 0.0] } 6... O-O { [%eval -0.08] } 7. Bg5 { [%eval -0.19] } 7... h6 { [%eval -0.29] } 8. Bxf6 { [%eval -0.36] } 8... Bxf6 { [%eval -0.37] } 9. O-O { [%eval -0.36] } 9... c6 { [%eval -0.12] } 10. Re1 { [%eval -0.17] } 10... Bf5 { [%eval -0.04] } 11. c4?! { [%eval -0.67] } 11... dxc4 { [%eval -0.5] } 12. Bxc4 { [%eval -0.77] } 12... Nd7?! { [%eval -0.1] } 13. Nc3 { [%eval 0.0] } 13... Nb6 { [%eval 0.0] } 14. b3?! { [%eval -0.76] } 14... Nxc4 { [%eval -0.49] } 15. bxc4 { [%eval -0.65] } 15... Qa5 { [%eval -0.55] } 16. Rc1 { [%eval -0.79] } 16... Rad8 { [%eval -0.78] } 17. d5?? { [%eval -5.41] } 17... Bxc3 { [%eval -5.42] } 18. Re5? { [%eval -7.61] } 18... Bxe5 { [%eval -7.78] } 19. Nxe5 { [%eval -7.72] } 19... cxd5 { [%eval -7.81] } 20. Qe1? { [%eval -9.29] } 20... Be6?? { [%eval 3.71] } 21. Rd1?? { [%eval -12.34] } 21... dxc4 { [%eval -12.71] } 22. Rxd8?! { [%eval #-1] } 22... Rxd8?! { [%eval -13.06] } 23. Qc3?! { [%eval #-2] } 23... Qxc3?! { [%eval #-4] } 24. g3 { [%eval #-3] } 24... Rd1+?! { [%eval #-4] } 25. Kg2 { [%eval #-4] } 25... Qe1?! { [%eval #-4] } 26. Kf3 { [%eval #-3] } 26... Qxe5 { [%eval #-2] } 27. Kg2 { [%eval #-2] } 27... Bd5+?! { [%eval #-2] } 28. Kh3 { [%eval #-1] } 28... Qh5# 0-1";

    let mut reader = BufferedReader::new_cursor(&pgn[..]);

    let mut visitor = LastPosition::new();
    let pos = reader.read_game(&mut visitor)?;

    println!("{:?}", pos);
    Ok(())
} */

impl<R: BufRead> Iterator for PgnRecordIterator<R> {
    type Item = PgnRecord;

    fn next(&mut self) -> Option<Self::Item> {
        let mut pgn_data = String::new();
        let mut white_elo = None;
        let mut black_elo = None;
        let mut result = None;

        while let Ok(line) = self.reader.read_line(&mut pgn_data) {
            //println!("{:?}, {:?}", line, pgn_data);
            if let Some(field_value) = extract_field_value(&pgn_data, "WhiteElo") {
                white_elo = field_value.parse::<u16>().ok();
            } else if let Some(field_value) = extract_field_value(&pgn_data, "BlackElo") {
                black_elo = field_value.parse::<u16>().ok();
            } else if let Some(field_value) = extract_field_value(&pgn_data, "Result") {
                result = Some(get_result(field_value));

            } else if pgn_data.starts_with("1.") {
                let tnotation_wrapped = to_tnotation(&pgn_data);
                if let Some(tnotation) = tnotation_wrapped {               
                    return Some(PgnRecord {
                        tnotation : tnotation, //result.0.to_string(),
                        white_elo : white_elo.unwrap(),
                        black_elo : black_elo.unwrap(),
                        result : result.unwrap(), //TODO
                    });
                }
            }
            pgn_data.clear();
        }
        return None;
    }
}
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

fn get_result(input: &str) -> u8 {
    if input == "0-1" {
        return 0;
    } else if input == "1-0" {
        return 2;
    } else if input == "1/2-1/2" {
        return 1;
    }
    panic!("Unknown result: {}", input);
}

fn get_last_n_chars(input: &str, n: usize) -> &str {
    let len = input.chars().count();
    if n >= len {
        return input;
    }
    let start_index = len - n;
    &input[start_index..]
}


fn extract_field_value<'a>(line: &'a str, field: &'a str) -> Option<&'a str> {
    let field_start = line.find(field)?;
    let value_start = field_start + field.len() + 2;
    let value_end = line[value_start..].find('"')? + value_start;
    Some(&line[value_start..value_end])
}

pub fn pgn_records(file_path: &str) -> io::Result<PgnRecordIterator<BufReader<File>>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    Ok(PgnRecordIterator { reader })
}



