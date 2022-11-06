use crate::{
	  code::{Chunk, OpCode},
	  disassemble::{print_value, disassemble_instruction},
	  value::Value
};

pub struct VM {
	  pub chunk: Chunk,
	  ip: usize,
	  stack: Vec<Value>,
}

#[derive(Debug)]
pub enum InterpretResult {
	  InterpretOk,
	  InterpretCompileError,
	  InterpretRuntimeError,
}

impl VM {
	  pub fn new(chunk: Chunk) -> VM {
		  VM {chunk, ip: 0, stack: vec![]}
	  }

	  pub fn interpret(&mut self) -> InterpretResult {
		  loop {
			  let instruction = self.chunk.code[self.ip];
			  self.ip += 1;

			  #[cfg(debug_assertions)]
			  {
				  print!("\nstack: ");
				  for value in &self.stack {
					  print!("[{}] ", print_value(*value))
				  }
				  println!("");
				  disassemble_instruction(&self.chunk, self.ip - 1);
				  println!("");
			  }

			  match instruction {
				  OpCode::OpReturn => return {
					  println!("{}", print_value(self.stack.pop().unwrap()));
					  InterpretResult::InterpretOk
				  },
				  OpCode::OpConstant => {
					  if let OpCode::_Value(constant) = self.chunk.code[self.ip] {
						  let value = self.chunk.constants[constant];
						  self.ip += 1;
						  self.stack.push(value);
						  println!("{}", print_value(value));
					  }
				  },
				  _ => return InterpretResult::InterpretRuntimeError
			  }
		  }
	  }
}
