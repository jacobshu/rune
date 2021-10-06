use std::fmt;

#[derive(Debug, Clone)]
pub enum TokenType {
  // One character
  LeftParen,
  RightParen,
  LeftBrace,
  RightBrace,
  Comma,
  Dot,
  Plus,
  Minus,
  Semicolon,
  Slash,
  Star,

  // One to two characters
  Bang,
  BangEqual,
  Equal,
  EqualEqual,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,

  // Literals
  Identifier,
  String,
  Number,

  // Keywords
  And,
  Class,
  Else,
  False,
  Fun,
  For,
  If,
  Nil,
  Or,
  Print,
  Return,
  Super,
  This,
  True,
  Var,
  While,

  EOF,
}

#[derive(Debug, Clone)]
pub enum Literal {
  Identifier(String),
  Str(String),
  Number(f64),
}

#[derive(Clone)]
pub struct Token {
  pub r#type: TokenType,
  pub lexeme: String,
  pub literal: Option<Literal>,
  pub line: Option<usize>,
  pub col: Option<u64>,
}

impl fmt::Debug for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "Token: {{ type: {:?}, lexeme: \"{}\", literal: {:?}, line: {:?}, col: {:?}}}",
      self.r#type, self.lexeme, self.literal, self.line, self.col
    )
  }
}

impl Token {
  pub fn new(token_type: TokenType, lexeme: &String) -> Self {
    Token {
      r#type: token_type,
      lexeme: lexeme.to_string(),
      literal: None,
      line: None,
      col: None,
    }
  }

  pub fn literal(self, literal: Literal) -> Self {
    Self {
      r#type: self.r#type,
      lexeme: self.lexeme,
      literal: Some(literal),
      line: None,
      col: None,
    }
  }

  pub fn line(self, line: usize) -> Self {
    Self {
      r#type: self.r#type,
      lexeme: self.lexeme,
      literal: self.literal,
      line: Some(line),
      col: None,
    }
  }

  pub fn col(self, col: u64) -> Self {
    Self {
      r#type: self.r#type,
      lexeme: self.lexeme,
      literal: self.literal,
      line: self.line,
      col: Some(col),
    }
  }

  pub fn build(self) -> Token {
    Token {
      r#type: self.r#type,
      lexeme: self.lexeme,
      literal: self.literal,
      line: self.line,
      col: self.col,
    }
  }
}

#[test]
fn build_token() {
  let raw_string = String::from("54");
  let row = 3;
  let column = 5;
  let token = Token::new(TokenType::String, &raw_string)
    .literal(Literal::Str(raw_string))
    .line(row)
    .col(column)
    .build();
  
  assert_eq!(token.r#type, TokenType::String);
  assert_eq!(token.lexeme, raw_string);
  assert_eq!(token.line.unwrap(), row);
  assert_eq!(token.col.unwrap(), column);
  // TODO: figure out where to parse literal values: either at token creation, or when inserting tokens on scanning
  // assert_eq!(token.literal, raw_string) 
}
