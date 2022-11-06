mod code;
mod disassemble;
mod value;
mod vm;

use code::{Chunk, OpCode};
use vm::VM;

fn main() {
	  let mut chunk = Chunk::new();

	  let constant = chunk.add_constant(1.2);
	  chunk.write_chunk(OpCode::OpConstant, 1);
	  chunk.write_chunk(OpCode::_Value(constant), 1);

	  chunk.write_chunk(OpCode::OpNegate, 1);

	  chunk.write_chunk(OpCode::OpReturn, 2);

	  let mut vm = VM::new(chunk);
	  let res = vm.interpret();

	  #[cfg(debug_assertions)]
	  println!("Result: {:?}", res);
}
