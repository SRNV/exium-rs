mod exium_context;
use exium_context::ExiumContext;
mod enums; use enums::{ Reason };

pub type onErrorType = fn (
  reason: Reason,
  cursor: Cursor,
  context: ExiumContext,
);
pub struct Exium {
  cursor: Cursor,
  char: char,
  char_code: u32,
  chars: Vec<char>,
  on_error: onErrorType,
  current_contexts: Vec<ExiumContext>,
}
impl Exium {
  pub fn new(on_error: onErrorType) -> Self {
    Exium {
      cursor: Cursor::new(0),
      char: ' ',
      char_code: ' ' as u32,
      chars: [].to_vec(),
      current_contexts: Vec::new(),
      on_error,
    }
  }
  pub fn read(mut self, source: &str) -> Vec<ExiumContext> {
    // save all chars
    self.chars = Vec::new();
    for char in source.chars() {
      self.chars.push(char);
    }
    while !self.is_end_of_file() {
      println!("{} {}", self.char, self.char_code);
      // shift
      self.shift(1);
    }
    self.current_contexts
  }
  fn shift(&mut self, x: i32) {
    self.cursor.x+= x;
    let size = self.chars.len();
    if self.cursor.x < size as i32 {
      self.assign_char();
    }
  }
  fn assign_char(&mut self) {
    let char: char = self.chars[self.cursor.x as usize];
    self.char = char;
    self.char_code = char as u32;
  }
  fn is_end_of_file(&mut self) -> bool {
    let size = self.chars.len();
    self.cursor.x >= size as i32
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