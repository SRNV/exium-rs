use crate::exium::enums::ContextTypes;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

pub trait ExiumContext : serde::de::Deserialize<'static> {
  fn new(self, _type: ContextTypes, source: String, start: i32) -> Self;
}

#[wasm_bindgen]
#[derive(Deserialize, Serialize)]
pub struct ExiumContextStruct {
  // position of the Context, 0 is the begining of the document
  start: i32,
  // the source of the Context
  source: String,
  // the type of the Context
  _type: ContextTypes,
  children: Vec<ExiumContextStruct>,
  related: Vec<ExiumContextStruct>,
  data: HashMap<String, bool>,
}
impl ExiumContext for ExiumContextStruct {
  fn new(self, _type: ContextTypes, source: String, start: i32) -> ExiumContextStruct {
    ExiumContextStruct {
      source,
      start,
      _type,
      children: Vec::new(),
      related: Vec::new(),
      data: HashMap::new(),
    }
  }
}
