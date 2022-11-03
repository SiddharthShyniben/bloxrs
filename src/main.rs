mod code;
mod disassemble;
mod value;

use code::{Chunk, OpCode};
use disassemble::dissasemble_chunk;

fn main() {
	  let mut chunk = Chunk::new();
	  chunk.push(OpCode::OpReturn);
	  dissasemble_chunk(&chunk, "test chunk")
}
