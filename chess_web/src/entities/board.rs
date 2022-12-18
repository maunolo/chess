use super::position::Position;
use super::stone::Stone;

pub fn fen_to_passant(field: &str) -> Option<Position> {
    if field == "-" {
        return None;
    }

    let x = field.chars().nth(0).unwrap() as i32 - 97;
    let y = 8 - field.chars().nth(1).unwrap().to_digit(10).unwrap() as i32;
    Some(Position { x, y })
}

pub fn fen_to_castle_rules(field: &str) -> CastleRules {
    let mut castle_rules = CastleRules {
        white: vec![],
        black: vec![]
    };

    field.chars().for_each(|c| {
        if c == 'K' {
            castle_rules.white.push(CastleOptions::CanCastleKingside);
        } else if c == 'Q' {
            castle_rules.white.push(CastleOptions::CanCastleQueenside);
        } else if c == 'k' {
            castle_rules.black.push(CastleOptions::CanCastleKingside);
        } else if c == 'q' {
            castle_rules.black.push(CastleOptions::CanCastleQueenside);
        }
    });

    castle_rules
}

pub fn fen_to_stones(field: &str) -> Vec<Vec<Option<Stone>>> {
    field.split("/").map(|row|
        row.chars().flat_map(|c| {
            if c.is_digit(10) {
                let n = c.to_digit(10).unwrap();
                (0..n).map(|_| '_').collect::<Vec<char>>()
            } else {
                vec![c]
            }
        }).map(|c| {
            match Stone::from_char(c) {
                Some(stone) => Some(stone),
                None => None
            }
        }).collect()
    ).collect()
}

pub enum CastleOptions {
  CanCastleKingside,
  CanCastleQueenside
}
pub struct CastleRules {
  white: Vec<CastleOptions>,
  black: Vec<CastleOptions>
}

pub struct Board {
  pub fen: String,
  pub pieces: Vec<Vec<Option<Stone>>>,
  pub turn: String,
  pub castle_rules: CastleRules,
  pub passant: Option<Position>,
  pub half_move_clock: i32,
  pub full_move_clock: i32
}

impl Board {
  pub fn new(fen: &str) -> Self {
      let fen_fields = fen.split(" ").collect::<Vec<&str>>();        
      Self {
          fen: fen.to_string(),
          pieces: fen_to_stones(fen_fields[0]),
          turn: fen_fields[1].to_string(),
          castle_rules: fen_to_castle_rules(fen_fields[2]),
          passant: fen_to_passant(fen_fields[3]),
          half_move_clock: fen_fields[4].parse::<i32>().unwrap(),
          full_move_clock: fen_fields[5].parse::<i32>().unwrap()
      }
  }

  pub fn get(&self, position: &Position) -> Option<Stone> {
      self.pieces[position.y as usize][position.x as usize].clone()
  }
}