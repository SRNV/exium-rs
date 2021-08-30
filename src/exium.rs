pub mod exium_context;
use exium_context::{ExiumContextStruct};
mod enums; use enums::{ Reason };
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[wasm_bindgen]
#[derive(Deserialize, Serialize)]
pub struct Exium {
  cursor: Cursor,
  char: char,
  char_code: u32,
  chars: Vec<char>,
  reason: Reason,
  current_contexts: Vec<ExiumContextStruct>,
}
impl Exium {
  pub fn new() -> Self {
    Exium {
      cursor: Cursor::new(0),
      char: ' ',
      char_code: ' ' as u32,
      reason: Reason::None,
      chars: [].to_vec(),
      current_contexts: Vec::new(),
    }
  }
  pub fn read(mut self, source: &str) -> Vec<ExiumContextStruct> {
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
#[derive(Deserialize, Serialize)]
pub struct Cursor {
  x: i32
}
impl Cursor {
  pub fn new(x: i32) -> Cursor {
    Cursor { x, }
  }
}