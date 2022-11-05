mod code;
mod disassemble;
mod value;

use code::{Chunk, OpCode};
use disassemble::dissasemble_chunk;

fn main() {
	  let mut chunk = Chunk::new();

	  let constant = chunk.add_constant(1.2);
	  chunk.write_chunk(OpCode::OpConstant, 1);
	  chunk.write_chunk(OpCode::_Value(constant), 1);

	  chunk.write_chunk(OpCode::OpReturn, 2);

	  dissasemble_chunk(&chunk, "test chunk")
}
