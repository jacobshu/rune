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

    // One to two characters
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Star,
    StarStar,

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
    pub fn new(token_type: TokenType, lexeme: &String) -> Token {
        Token {
            r#type: token_type,
            lexeme: lexeme.to_string(),
            literal: None,
            line: None,
            col: None,
        }
    }

    pub fn literal<'a>(&'a mut self, literal: Literal) -> &'a mut Token {
        self.literal = Some(literal);
        self
    }

    pub fn line<'a>(&'a mut self, line: usize) -> &'a mut Token {
        self.line = Some(line);
        self
    }

    pub fn col<'a>(&'a mut self, col: u64) -> &'a mut Token {
        self.col = Some(col);
        self
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
