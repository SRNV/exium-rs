pub struct Exium {
  cursor: Cursor,
  char: char,
  char_code: u32,
}
impl Exium {
  pub fn new() -> Self {
    Exium {
      cursor: Cursor::new(0),
      char: ' ',
      char_code: ' ' as u32,
    }
  }
  pub fn read(mut self, source: &str) {
    let source_iter = source.chars();
    for (i, char) in source_iter.enumerate() {
      println!("{} {}", self.char, self.char_code);
      let new_x = i as i32;
      // assign char and char code
      self.assign_char(char);
      // shift
      self.shift(
        new_x
      );
    }
  }
  fn shift(&mut self, x: i32) {
    self.cursor.x = x;
  }
  fn assign_char(&mut self, char: char) {
    self.char = char;
    self.char_code = char as u32;
  }
}
pub struct Cursor {
  x: i32
}
impl Cursor {
  pub fn new(x: i32) -> Cursor {
    Cursor { x, }
  }
}