mod code;
mod disassemble;
mod value;
mod vm;

use code::{Chunk, OpCode};
use vm::VM;

use crate::disassemble::dissasemble_chunk;

fn main() {
	  let mut chunk = Chunk::new();

	  let constant = chunk.add_constant(1.2);
	  chunk.write_chunk(OpCode::OpConstant, 1);
	  chunk.write_chunk(OpCode::_Value(constant), 1);

	  let constant = chunk.add_constant(3.4);
	  chunk.write_chunk(OpCode::OpConstant, 1);
	  chunk.write_chunk(OpCode::_Value(constant), 1);

	  chunk.write_chunk(OpCode::OpAdd, 1);

	  let constant = chunk.add_constant(5.6);
	  chunk.write_chunk(OpCode::OpConstant, 2);
	  chunk.write_chunk(OpCode::_Value(constant), 2);

	  chunk.write_chunk(OpCode::OpDivide, 2);
	  chunk.write_chunk(OpCode::OpNegate, 3);

	  chunk.write_chunk(OpCode::OpReturn, 4);

	  #[cfg(debug_assertions)]
	  dissasemble_chunk(&chunk, "test chunk");

	  let mut vm = VM::new(chunk);
	  let res = vm.interpret();

	  #[cfg(debug_assertions)]
	  println!("Result: {:?}", res);
}
