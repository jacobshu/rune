use std::fmt;

#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
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
    Fn,
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

    Whitespace,
    Comment,
    EOF,
    Invalid,
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
    pub line: usize,
    pub col: usize,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Token: {{ type: {:?}, lexeme: \"{}\", line: {:?}, col: {:?}}}",
            self.r#type, self.lexeme, self.line, self.col
        )
    }
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &String, line: usize, col: usize) -> Token {
        Token {
            r#type: token_type,
            lexeme: lexeme.to_string(),
            line,
            col,
        }
    }
}

#[test]
fn build_token() {
    let raw_string = String::from("54");
    let row = 3;
    let column = 5;
    let mut token = Token::new(TokenType::String, &raw_string, row, column);

    assert_eq!(token.r#type, TokenType::String);
    assert_eq!(token.lexeme, raw_string);
    assert_eq!(token.line, row);
    assert_eq!(token.col, column);
    // TODO: figure out where to parse literal values: either at token creation, or when inserting tokens on scanning
    // assert_eq!(token.literal, raw_string)
}
