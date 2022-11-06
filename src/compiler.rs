use crate::{scanner::{Scanner, TokenType}, code::Chunk};

pub struct Compiler {}

impl Compiler {
	  pub fn new() -> Compiler {
		  Compiler {}
	  }

	  pub fn compile(&self, source: String, chunk: &Chunk) -> bool {
		  let mut scanner = Scanner::new(source);
	  }
}
