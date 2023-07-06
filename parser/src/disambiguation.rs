use shakmaty::{Chess, Position, Square, CastlingSide};
use pgn_reader::{Visitor, Skip, BufferedReader, SanPlus, San};

struct Role(shakmaty::Role); // wrapper for the Role enum...

impl Role {

    pub fn to_char(role : Option<Role>) -> char {
        if role.is_none() {
            return '-';
        }
        return role.unwrap().0.char();
    }

    fn to_string(role : Option<Role>) -> String {
        return Role::to_char(role).to_string();
    }
}

impl From<shakmaty::Role> for Role {
    fn from(value: shakmaty::Role) -> Self {
        return Role(value);
    }
}

impl From<Role> for shakmaty::Role {
    fn from(value: Role) -> Self {
        return value.0;
    }
}

impl From<Role> for String {
    fn from(value: Role) -> Self {
        return Role::to_string(Some(value));
    }
}

impl From<Role> for u16 {
    fn from(value: Role) -> Self {
        return value.into();
    }
}

impl From<u16> for Role {
    fn from(value: u16) -> Self {
        let role = shakmaty::Role ::try_from(value);
        return role.unwrap().into();
    }
}

pub struct TNotation<T> {
    pos: Chess,
    result : T,
}

impl<T> TNotation<T> {

    fn new(result : T) -> TNotation<T> {
        TNotation { pos: Chess::default(), result : result }
    }

    fn castle_position(san : &San, from : &Square, to : &Square) -> Square {
        // there is a bug in `to` when castling... replace characters for the correct ones.
        match (&san, from) {
            (San::Castle(CastlingSide::QueenSide), Square::E8) => { Square::C8 },
            (San::Castle(CastlingSide::KingSide), Square::E8) => { Square::G8 },
            (San::Castle(CastlingSide::QueenSide), Square::E1) => { Square::C1 },
            (San::Castle(CastlingSide::KingSide), Square::E1) => { Square::G1 },
            (San::Normal { .. }, _) => { *to },
            invalid => unreachable!("Invalid move: {:?}", invalid),
        }
    }

    fn from_bits(&self, bits : u16) -> String {
        let mut result = String::new();
        let pawn_prom : bool = (bits & 1) != 0; // extract first bit
        let mut mask = ((1 << (3 - 1 + 1)) - 1) << 1;
        let role : String = String::from(Role::from((bits & mask) >> 1));
        if pawn_prom { // next bits are promotion bits
            result.push_str(format!("p{}", role).as_str()); // promotion, the piece was a pawn
        } else { // next bits are piece
            result.push_str(format!("{}-", role).as_str());
        }
        mask = ((1 << (9 - 4 + 1)) - 1) << 4;
        let from : Square = Square::new(((bits & mask) >> 4) as u32);
        mask = ((1 << (15 - 10 + 1)) - 1) << 10;
        let to : Square = Square::new(((bits & mask) >> 10) as u32);
        result.push_str(format!("{}{}", from, to).as_str());
        return result;
    }

    fn to_string(&self, san_plus : &SanPlus) -> String {
        let m = san_plus.san.to_move(&self.pos).unwrap();
        let role : String = Role(m.role()).into();
        let mut to = m.to();
        let from = m.from().unwrap();
        to = TNotation::<T>::castle_position(&san_plus.san, &from, &to);
        let promotion: String  = Role::to_string(m.promotion().map(Into::into));
        return format!("{}{}{}{}", role, promotion, from, to);
    }

    fn to_bits(&self, san_plus: &SanPlus) -> u16 {
        // BINARY NOTATION
        // 1 bit indicates pawn promotion (1=promotion see below)
        // 3 bits indicates piece OR pawn promotion choice
        // 6 bits from
        // 6 bits to
        // u16!
        let m = san_plus.san.to_move(&self.pos).unwrap();
        let role : u16 = m.role().into();         // 3 bits
        let _from = m.from().unwrap();
        let _to = m.to();     
        let to = TNotation::<T>::castle_position(&san_plus.san, &_from, &_to) as u16;   // 6 bits
        let from = _from as u16;                                             // 6 bits
        let promotion = match m.promotion() { None => 0, Some(x) => x as u16 }; //3 bits promote to...? 
        let mut y = 0;
        if promotion == 0 {
            y = (role << 1) + (from << 4) + (to << 10);
        } else {
            y = 1 + (promotion << 1) + (from << 4) + (to << 10);
        }
        return y;
    }

    pub fn consume(self) -> T {
        return self.result;
    }
}

impl Visitor for TNotation<String> {
    type Result = bool;

    fn begin_variation(&mut self) -> Skip {
        Skip(true) // stay in the mainline
    }

    fn san(&mut self, san_plus: SanPlus) {
        let x = self.to_string(&san_plus);
        let san = san_plus.san;
        let m = san.to_move(&self.pos).unwrap();
        self.result.push_str(format!("{} ", x).as_str());
        self.pos.play_unchecked(&m);
    }

    fn end_game(&mut self) -> Self::Result {
        return true; // did anything go wrong parsing?
    }
}

impl Visitor for TNotation<Vec<u16>> {
    type Result = bool;

    fn begin_variation(&mut self) -> Skip {
        Skip(true) // stay in the mainline
    }
    fn san(&mut self, san_plus: SanPlus) {
        let x = self.to_bits(&san_plus);
        let san = san_plus.san;
        let m = san.to_move(&self.pos).unwrap();
        self.result.push(x);
        self.pos.play_unchecked(&m);
    }

    fn end_game(&mut self) -> Self::Result {
        return true;
    }
}


pub fn parse<X>(pgn_data : &str,  data : X) -> Option<X> where TNotation<X> : Visitor {
    //let data = record_parser.new_data();
    let mut reader = BufferedReader::new_cursor(pgn_data);
    let mut visitor = TNotation::new(data);
    reader.read_game(&mut visitor).unwrap(); // check result
    return Some(visitor.consume());
}

/*
pub fn parse<X, T : TNotationParse<X>>(pgn_data : &str,  record_parser : &T) -> Option<X> where TNotation<X> : Visitor {
    let data = record_parser.new_data();
    let mut reader = BufferedReader::new_cursor(pgn_data);
    let mut visitor = TNotation::new(data);
    reader.read_game(&mut visitor).unwrap(); // check result
    return Some(visitor.consume());
} */

