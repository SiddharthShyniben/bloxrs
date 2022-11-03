pub enum OpCode {
	  OpReturn = 0
}

pub struct Chunk {
	  code: Vec<u8>
}

impl Chunk {
	  pub fn new() -> Chunk {
		  Chunk {code: vec![]}
	  }

	  pub fn write(&mut self, code: OpCode) {
		  self.code.push(code as u8);
	  }

	  pub fn count(&self) -> usize {
		  self.code.len()
	  }
}
