mod code;
mod disassemble;

use code::{Chunk, OpCode};
use disassemble::dissasemble_chunk;

fn main() {
	  let mut chunk = Chunk::new();
	  chunk.write(OpCode::OpReturn);
	  dissasemble_chunk(&chunk, "test chunk")
}
