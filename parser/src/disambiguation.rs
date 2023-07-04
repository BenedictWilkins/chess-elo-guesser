
use shakmaty::{Chess, Position, Role};
use pgn_reader::{Visitor, Skip, BufferedReader, SanPlus};

struct TNotation {
    pos: Chess,
    result : String,
}

impl TNotation {
    fn new() -> TNotation {
        TNotation { pos: Chess::default(), result : String::new() }
    }

    fn capture_notation(capture : Option<Role>) -> &'static str {
        if capture.is_none() {
            return "x";
        } else {
            return ""
        }
    }

    fn role_notation(role : Option<Role>) -> String {
        if role.is_none() {
            return "".to_string();
        }
        return role.unwrap().char().to_string();
    }
}

impl Visitor for TNotation {
    type Result = String;

    fn begin_variation(&mut self) -> Skip {
        Skip(true) // stay in the mainline
    }

    fn san(&mut self, san_plus: SanPlus) {
        //println!("{:?}", san_plus);
        let san = san_plus.san;
        let m = san.to_move(&self.pos).unwrap();
        let role = TNotation::role_notation(Some(m.role()));
        let to = m.to();
        let from = m.from().unwrap();
        let capture = TNotation::capture_notation(m.capture());
        let promotion = TNotation::role_notation(m.promotion());
        let x = format!("{}{}{}{}{} ", role, capture, to, from, promotion);
        self.result.push_str(&x);
        self.pos.play_unchecked(&m);
    }

    fn end_game(&mut self) -> Self::Result {
        return self.result.clone();
    }
}

pub fn to_tnotation(pgn_data : &str) -> Option<String> {
    let mut reader = BufferedReader::new_cursor(pgn_data);
    let mut visitor = TNotation::new();
    let result = reader.read_game(&mut visitor).unwrap();
    return result;
}

