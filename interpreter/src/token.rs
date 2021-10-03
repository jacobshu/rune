pub enum TokenTypes {
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

  EOF

}

#[derive(Debug, Clone)]
pub enum Literal {
  Identifier(String),
  Str(String),
  Number(f64),
}

#[derive(Clone)]
pub struct Token {
  r#type: TokenType,
  lexeme: String,
  literal: Option<Literal>
  line: usize,
  col: u64,
}

impl fmt::Debug for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Token: { type: {:?}, lexeme: \"{}\", literal: {:?}, line: {:?}, col: {:?}}",
        self.r#type,
        String::from_utf8(self.lexeme.clone()).unwrap(),
        self.literal,
        self.line,
        self.col
    )
  }
}