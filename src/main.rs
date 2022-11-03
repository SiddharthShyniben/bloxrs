mod code;
mod disassemble;

use code::{Chunk, OpCode};

fn main() {
	  let mut chunk = Chunk::new();
	  chunk.write(OpCode::OpReturn);
}
