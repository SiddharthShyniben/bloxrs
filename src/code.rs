use crate::value::{Value, Values};

#[derive(Copy, Clone, Debug)]
pub enum OpCode {
	  OpConstant,
	  OpAdd,
	  OpSubtract,
	  OpMultiply,
	  OpDivide,
	  OpNegate,
	  OpReturn,
	  _Value(usize),
}

pub type Code = Vec<OpCode>;

pub struct Chunk {
	  pub code: Code,
	  pub constants: Values,
	  pub lines: Vec<isize>,
}

impl Chunk {
	  pub fn new() -> Chunk {
		  Chunk {
			  code: Code::new(),
			  constants: Values::new(),
			  lines: vec![],
		  }
	  }

	  pub fn write_chunk(&mut self, code: OpCode, line: isize) {
		  self.code.push(code);
		  self.lines.push(line);
	  }

	  pub fn add_constant(&mut self, value: Value) -> usize {
		  self.constants.push(value);
		  self.constants.len() - 1
	  }
}
