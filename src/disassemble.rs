use crate::{code::{Chunk, OpCode}, value::Value};

pub fn dissasemble_chunk(chunk: &Chunk, name: &str) {
	  println!("== {: ^19} ==", name);

	  let mut offset = 0usize;

	  while offset < chunk.code.len() {
		  offset = disassemble_instruction(chunk, offset);
	  }

	  println!("== {: ^19} ==", " ".repeat(name.len()));
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
	  print!("{:0>4} ", offset);

	  if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
		  print!("    | ");
	  } else {
		  print!(" {:0>2} | ", chunk.lines[offset]);
	  }

	  let instruction = chunk.code[offset];
	  return match instruction {
		  OpCode::OpConstant => constant_instruction("OP_CONSTANT", chunk, offset),
		  OpCode::OpNegate => simple_instruction("OP_NEGATE", offset),
		  OpCode::OpReturn => simple_instruction("OP_RETURN", offset),
		  _ => {
			  print!("Unknown opcode {:?}", instruction);
			  offset + 1
		  }
	  }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
	  print!("{: >15}\n", name);
	  offset + 1
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
	  let constant = chunk.code[offset + 1];
	  print!("{: >15} ", name);

	  if let OpCode::_Value(c) = constant {
		  print!("({:0>3}) {}\n", c, print_value(chunk.constants[c]));
	  }

	  offset + 2
}

pub fn print_value(value: Value) -> String {
	  format!("{}", value)
}
