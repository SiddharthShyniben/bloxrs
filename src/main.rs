mod code;
mod disassemble;
mod value;
mod vm;
mod compiler;
mod scanner;

use code::{Chunk, OpCode};
use promptly::prompt;
use vm::{VM, InterpretResult};
use crate::disassemble::dissasemble_chunk;

use std::{env, process, fs};

fn main() {
	  let args: Vec<String> = env::args().collect();

	  match args.len() {
		  1 => repl(),
		  2 => run_file(&args[1]),
		  _ => {
			  eprintln!("Usage: loxrs [path]");
			  process::exit(64);
		  }
	  }
}

fn repl() {
	  let vm = VM::new();
	  loop {
		  let line: String = prompt("::").unwrap();
		  println!("{:?}", line);
		  vm.interpret(line);
	  }
}

fn run_file(file: &String) {
	  let contents = fs::read_to_string(file)
		  .expect("Should have been able to read the file");

	  let vm = VM::new();
	  let result = vm.interpret(contents);

	  match result {
		  InterpretResult::InterpretCompileError => process::exit(65),
		  InterpretResult::InterpretRuntimeError => process::exit(65),
		  InterpretResult::InterpretOk => process::exit(0),
	  }
}
