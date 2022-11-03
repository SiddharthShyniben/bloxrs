#[derive(Copy, Clone)]
pub enum OpCode {
	  OpReturn = 0
}

pub struct Chunk {
	  code: Vec<OpCode>
}

impl Chunk {
	  pub fn new() -> Chunk {
		  Chunk {code: vec![]}
	  }

	  pub fn write(&mut self, code: OpCode) {
		  self.code.push(code);
	  }

	  pub fn length(&self) -> usize {
		  self.code.len()
	  }

	  pub fn at(&self, n: usize) -> OpCode {
		  self.code[n]
	  }
}
