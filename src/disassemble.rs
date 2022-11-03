use crate::code::{Chunk, OpCode};

pub fn dissasemble_chunk(chunk: &Chunk, name: &str) {
	  println!("== {} ==", name);

	  let mut offset = 0usize;

	  while offset < chunk.length() {
		  offset = disassemble_instruction(chunk, offset);
	  }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
	  print!("{} ", format!("{:0>4}", offset));

	  let instruction = chunk.at(offset);

	  return match instruction {
		  OpCode::OpReturn => simple_instruction("OP_RETURN", offset),
		  _ => {
			  print!("Unknown opcode {}", instruction as u8);
			  offset + 1
		  }
	  }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
	  print!("{}\n", name);
	  offset + 1
}
