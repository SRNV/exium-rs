use crate::exium::Exium;
use crate::exium::enums::ContextTypes;

pub struct ExiumContext {
  // position of the Context, 0 is the begining of the document
  _x: i32,
  // the source of the Context
  _source: String,
  // the type of the Context
  _type: ContextTypes,
  _children: Vec<ExiumContext>,
  _related: Vec<ExiumContext>,
}
impl ExiumContext {}
