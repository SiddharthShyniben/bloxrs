use crate::scanner::{Scanner, TokenType};

pub struct Compiler {}

impl Compiler {
	  pub fn new() -> Compiler {
		  Compiler {}
	  }

	  pub fn compile(&self, source: String) {
		  let mut scanner = Scanner::new(source);
		  let mut line = -1;

		  'tokens: loop {
		      let token = scanner.scan_token();
			  if token.line != line {
				  print!("{:0>3} | ", token.line);
				  line = token.line;
			  } else {
				  print!("    | ");
			  }

			  print!("{:?}, '{}'", token.token_type, token.start);
			  if matches!(token.token_type, TokenType::Eof) {
				  break 'tokens
			  }
		  }
	  }
}
