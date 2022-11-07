use crate::{scanner::{Scanner, TokenType, Token}, code::Chunk};

struct Parser {
	  pub current: Option<Token>,
	  pub previous: Option<Token>,
}

impl Parser {
	  pub fn new() -> Parser {
		  Parser {current: None, previous: None}
	  }
}

pub struct Compiler {
	  parser: Parser
}

impl Compiler {
	  pub fn new() -> Compiler {
		  Compiler {parser: Parser::new()}
	  }

	  pub fn compile(&self, source: String, chunk: &Chunk) -> bool {
		  let mut scanner = Scanner::new(source);
		  self.advance();
		  self.expression();
		  self.consume(TokenType::Eof, "Expect end of expression.");
		  true
	  }

	  fn advance(&mut self) {
		  self.parser.previous = self.parser.current;

		  loop {
			  self.parser.current = self.scan_token();

			  if let Some(token) = self.parser.current {
				  if !matches!(token.token_type, TokenType::Error) {
					  break
				  }

				  self.error_at_current(token.error.unwrap());
			  }
		  }
	  }


	  fn error_at_current(&self, message: String) {
		  self.error_at(self.parser.previous.unwrap(), message)
	  }

	  fn error_at(&self, token: Token, message: String) {
		  eprint!("[{}:{}] ERROR", token.line, token.column);

		  match token.token_type {
			  TokenType::Eof => eprint!(" at end"),
			  TokenType::Error => {},
			  _ => eprint!(" at '{}'", token.start)
		  }
	  }
}

