pub struct Scanner {
	  start: usize,
	  current: usize,
	  line: isize,
	  column: isize,
	  code: String
}

impl Scanner {
	  pub fn new(code: String) -> Scanner {
		  Scanner { start: 0, current: 0, line: 1, column: 1, code }
	  }

	  pub fn scan_token(&mut self) -> Token {
		  self.skip_whitespace();
		  self.start = self.current;

		  if self.is_at_end() {
			  return self.make_token(TokenType::Eof)
		  }

		  let c = self.advance();

		  if is_digit(c) {
			  return self.number();
		  }

		  if is_alpha(c) {
			  return self.identifier();
		  }

		  match c {
			  '(' => self.make_token(TokenType::LeftParen),
			  ')' => self.make_token(TokenType::RightParen),
			  '{' => self.make_token(TokenType::LeftBrace),
			  '}' => self.make_token(TokenType::RightBrace),
			  ';' => self.make_token(TokenType::Semicolon),
			  ',' => self.make_token(TokenType::Comma),
			  '.' => self.make_token(TokenType::Dot),
			  '+' => self.make_token(TokenType::Plus),
			  '-' => self.make_token(TokenType::Minus),
			  '*' => self.make_token(TokenType::Star),
			  '/' => self.make_token(TokenType::Slash),

			  '!' => self.make_token(if self._match('=') {TokenType::BangEqual} else {TokenType::Bang}),
			  '=' => self.make_token(if self._match('=') {TokenType::EqualEqual} else {TokenType::Equal}),
			  '>' => self.make_token(if self._match('=') {TokenType::GreaterEqual} else {TokenType::Greater}),
			  '<' => self.make_token(if self._match('=') {TokenType::LessEqual} else {TokenType::Less}),

			  '"' => self.string(),

			  _ => self.error_token("Unexpected character.".to_string())
		  }
	  }

	  fn identifier(&self) -> Token {
		  while is_alpha(self.peek()) || is_digit(self.peek()) {
			  self.advance();
		  }

		  self.make_token(self.identifier_type())
	  }

	  fn identifier_type(&self) -> TokenType {
		  TokenType::Identifier
	  }

	  fn string(&self) -> Token {
		  while self.peek() != '"' && !self.is_at_end() {
			  if self.peek() == '\n' {
				  self.line += 1;
				  self.column = 1;
			  } else {
				  self.column += 1;
			  }

			  self.advance();
		  }

		  if self.is_at_end() {
			  self.error_token("Unterminated string.".to_string())
		  } else {
			  self.advance();
			  self.make_token(TokenType::String)
		  }
	  }

	  fn number(&self) -> Token {
		  while is_digit(self.peek()) {
			  self.advance();
		  }

		  if self.peek() == '.' && is_digit(self.peek_next()) {
			  self.advance();

			  while is_digit(self.peek_next()) {
			      self.advance();
			  }
		  }

		  self.make_token(TokenType::Number)
	  }

	  fn is_at_end(&self) -> bool {
		  self.current == self.code.len()
	  }

	  fn make_token(&self, token_type: TokenType) -> Token {
		  Token {
			  token_type,
			  start: self.start,
			  length: self.current - self.start,
			  line: self.line,
			  column: self.column,
			  error: None
		  }
	  }

	  fn error_token(&self, msg: String) -> Token {
		  Token {
			  token_type: TokenType::Error,
			  start: self.start,
			  length: msg.len(),
			  line: self.line,
			  column: self.column,
			  error: Some(msg)
		  }
	  }

	  fn advance(&self) -> char {
		  self.current += 1;
		  self.column += 1;
		  self.code.as_bytes()[self.current - 1] as char
	  }

	  fn _match(&self, c: char) -> bool {
		  if self.is_at_end() {
			  false
		  } else if self.code.as_bytes()[self.current - 1] as char != c {
			  false
		  } else {
			  self.current += 1;
			  self.column += 1;
			  true
		  }
	  }

	  fn skip_whitespace(&self) {
		  loop {
			  let c = self.peek();
			  match c {
				  ' ' | '\r' | '\t' => {
					  self.advance();
					  self.column += 1;
				  }
				  '\n' => {
					  self.line += 1;
					  self.column = 1;
					  self.advance();
				  }
				  '/' => {
					  if self.peek_next() == '/' {
						  while self.peek() != '\n' && !self.is_at_end() {
							  self.advance();
						  }
					  }
				  }
				  _ => return
			  }
		  };
	  }

	  fn peek(&self) -> char {
	      self.code.as_bytes()[self.current] as char
	  }

	  fn peek_next(&self) -> char {
		  if self.is_at_end() {
			  '\0'
		  } else {
			  self.code.as_bytes()[self.current + 1] as char
		  }
	  }
}

#[derive(Debug)]
pub struct Token {
	  pub token_type: TokenType,
	  pub start: usize,
	  pub length: usize,
	  pub line: isize,
	  pub column: isize,
	  pub error: Option<String>
}

#[derive(Debug)]
pub enum TokenType {
	  // Single-character tokens.
	  LeftParen, RightParen,
	  LeftBrace, RightBrace,
	  Comma, Dot, Minus, Plus,
	  Semicolon, Slash, Star,
	  // One or two character tokens.
	  Bang, BangEqual,
	  Equal, EqualEqual,
	  Greater, GreaterEqual,
	  Less, LessEqual,
	  // Literals.
	  Identifier, String, Number,
	  // Keywords.
	  And, Class, Else, False,
	  For, Fun, If, Nil, Or,
	  Print, Return, Super, This,
	  True, Var, While,

	  Error, Eof
}

fn is_digit(c: char) -> bool {
	  c >= '0' && c <= '9'
}

fn is_alpha(c: char) -> bool {
	  (c >= 'a' && c <= 'z') ||
		  (c >= 'A' && c <= 'Z') ||
		  c == '_'
}
