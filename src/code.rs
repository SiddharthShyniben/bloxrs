#[derive(Copy, Clone)]
pub enum OpCode {
	  OpReturn = 0
}

pub type Chunk = Vec<OpCode>;
