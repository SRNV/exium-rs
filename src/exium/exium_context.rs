use crate::exium::Exium;
use crate::exium::enums::ContextTypes;
use std::collections::HashMap;
use std::any::{Any};

pub struct ExiumContext {
  // position of the Context, 0 is the begining of the document
  start: i32,
  // the source of the Context
  source: String,
  // the type of the Context
  _type: ContextTypes,
  children: Vec<ExiumContext>,
  related: Vec<ExiumContext>,
  data: HashMap<String, bool>,
}
impl ExiumContext {
  fn new(self, _type: ContextTypes, source: String, start: i32) -> Self {
    ExiumContext {
      source,
      start,
      _type,
      children: Vec::new(),
      related: Vec::new(),
      data: HashMap::new(),
    }
  }
}
