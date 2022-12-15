use std::str::FromStr;

#[derive(Clone)]
pub struct Position {
  pub x: i32,
  pub y: i32
}

impl Position {
  pub fn new(x: i32, y: i32, is_white_view: bool) -> Position {
    match is_white_view {
      true => Position { x, y },
      false => Position { x: 7 - x, y: 7 - y }
    }
  }

  pub fn to_string(&self) -> String {
    let buffer: [u8; 1] = [self.x as u8 + 97];
    format!("{}{}", std::str::from_utf8(&buffer).unwrap(), 8 - self.y)
  }
}

impl FromStr for Position {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let x = s.as_bytes()[0] as i32 - 97;
    let y = 8 - s.as_bytes()[1] as i32 - 48;
    Ok(Position { x, y })
  }
}