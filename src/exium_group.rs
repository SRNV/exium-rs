use crate::exium_sequence::Sequence;
use crate::exium::Exium;
use crate::context_types::ContextTypes;

pub struct ExiumGroup {
  // position of the group, 0 is the begining of the document
  _x: i32,
  // the source of the group
  _source: str,
  // the type of the group
  _type: ContextTypes,
  // opening char of the ExiumGroup
  _begin_with: char,
  // ending char of the ExiumGroup
  _end_with: char,
  _begin_sequence: Sequence,
  _end_sequence: Sequence,
  _end_exclude: Vec<char>,
  _children: Vec<ExiumGroup>,
  _related: Vec<ExiumGroup>,
  _exclude: Vec<char>,

  // if a char or a sequence is valid
  accept: fn(exium: Exium) -> boolean,
}
impl ExiumGroup {}
