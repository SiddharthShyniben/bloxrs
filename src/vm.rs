use crate::{
	  code::Chunk,
	  value::Value, compiler::Compiler,
};

pub struct VM {
	  pub chunk: Option<Chunk>,
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
	  pub fn new() -> VM {
		  VM {
			  chunk: None,
			  ip: 0,
			  stack: vec![],
		  }
	  }

	  pub fn interpret(&mut self, source: String) -> InterpretResult {
		  let compiler = Compiler::new();
		  compiler.compile(source);
		  InterpretResult::InterpretOk
	  }
}


// self.chunk = Some(chunk);
//
// if let Some(chunk) = &self.chunk {
// 	  loop {
// 		  let instruction = chunk.code[self.ip];
// 		  self.ip += 1;
//
// 		  #[cfg(debug_assertions)]
// 		  {
// 			  print!("\nstack: ");
// 			  for value in &self.stack {
// 				  print!("[{}] ", print_value(*value))
// 			  }
// 			  println!("");
// 			  disassemble_instruction(&chunk, self.ip - 1);
// 			  println!("");
// 		  }
//
// 		  match instruction {
// 			  OpCode::OpReturn => {
// 				  return {
// 					  println!("{}", print_value(self.stack.pop().unwrap()));
// 					  InterpretResult::InterpretOk
// 				  }
// 			  }
// 			  OpCode::OpNegate => {
// 				  let v = self.stack.pop().unwrap();
// 				  self.stack.push(-v);
// 			  }
// 			  OpCode::OpAdd => {
// 				  let b = self.stack.pop().unwrap();
// 				  let a = self.stack.pop().unwrap();
// 				  self.stack.push(a + b)
// 			  }
// 			  OpCode::OpSubtract => {
// 				  let b = self.stack.pop().unwrap();
// 				  let a = self.stack.pop().unwrap();
// 				  self.stack.push(a - b)
// 			  }
// 			  OpCode::OpMultiply => {
// 				  let b = self.stack.pop().unwrap();
// 				  let a = self.stack.pop().unwrap();
// 				  self.stack.push(a * b)
// 			  }
// 			  OpCode::OpDivide => {
// 				  let b = self.stack.pop().unwrap();
// 				  let a = self.stack.pop().unwrap();
// 				  self.stack.push(a / b)
// 			  }
// 			  OpCode::OpConstant => {
// 				  if let OpCode::_Value(constant) = chunk.code[self.ip] {
// 					  let value = chunk.constants[constant];
// 					  self.ip += 1;
// 					  self.stack.push(value);
// 				  }
// 			  }
// 			  _ => return InterpretResult::InterpretRuntimeError,
// 		  }
// 	  }
// } else {
// 	  InterpretResult::InterpretCompileError
// }
