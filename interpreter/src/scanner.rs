use crate::error::{ErrorType, RuneError};
use crate::token::{Token, TokenType};
use std::iter::Peekable;
use std::str::CharIndices;

pub struct Scanner<'a> {
    source: Peekable<CharIndices<'a>>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    last_newline: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a String) -> Self {
        Self {
            source: input.char_indices().peekable(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            last_newline: 0,
        }
    }

    fn is_at_end(&mut self) -> bool {
        self.source.peek() == None
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }
        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        while let Some((index, char)) = self.source.next() {
            let token = match char {
                '(' => Token::new(
                    TokenType::LeftParen,
                    &char.to_string(),
                    self.line,
                    index - self.last_newline,
                ),
                ')' => Token::new(
                    TokenType::RightParen,
                    &char.to_string(),
                    self.line,
                    index - self.last_newline,
                ),
                '{' => Token::new(
                    TokenType::LeftBrace,
                    &char.to_string(),
                    self.line,
                    index - self.last_newline,
                ),
                '}' => Token::new(
                    TokenType::RightBrace,
                    &char.to_string(),
                    self.line,
                    index - self.last_newline,
                ),
                ',' => Token::new(
                    TokenType::Comma,
                    &char.to_string(),
                    self.line,
                    index - self.last_newline,
                ),
                '.' => Token::new(
                    TokenType::Dot,
                    &char.to_string(),
                    self.line,
                    index - self.last_newline,
                ),
                '+' => Token::new(
                    TokenType::Plus,
                    &char.to_string(),
                    self.line,
                    index - self.last_newline,
                ),
                '-' => Token::new(
                    TokenType::Minus,
                    &char.to_string(),
                    self.line,
                    index - self.last_newline,
                ),
                ';' => Token::new(
                    TokenType::Semicolon,
                    &char.to_string(),
                    self.line,
                    index - self.last_newline,
                ),

                // // One to two characters
                '=' => match self.source.next_if_eq(&(index + 1, '=')) {
                    Some(_equals) => Token::new(
                        TokenType::EqualEqual,
                        &"==".to_string(),
                        self.line,
                        index - self.last_newline,
                    ),
                    None => Token::new(
                        TokenType::Equal,
                        &char.to_string(),
                        self.line,
                        index - self.last_newline,
                    ),
                },
                '!' => match self.source.next_if_eq(&(index + 1, '=')) {
                    Some(_equals) => Token::new(
                        TokenType::BangEqual,
                        &"!=".to_string(),
                        self.line,
                        index - self.last_newline,
                    ),
                    None => Token::new(
                        TokenType::Bang,
                        &char.to_string(),
                        self.line,
                        index - self.last_newline,
                    ),
                },
                '>' => match self.source.next_if_eq(&(index + 1, '=')) {
                    Some(_equals) => Token::new(
                        TokenType::GreaterEqual,
                        &">=".to_string(),
                        self.line,
                        index - self.last_newline,
                    ),
                    None => Token::new(
                        TokenType::Greater,
                        &char.to_string(),
                        self.line,
                        index - self.last_newline,
                    ),
                },
                '<' => match self.source.next_if_eq(&(index + 1, '=')) {
                    Some(_equals) => Token::new(
                        TokenType::LessEqual,
                        &"<=".to_string(),
                        self.line,
                        index - self.last_newline,
                    ),
                    None => Token::new(
                        TokenType::Less,
                        &char.to_string(),
                        self.line,
                        index - self.last_newline,
                    ),
                },
                '*' => match self.source.next_if_eq(&(index + 1, '*')) {
                    Some(_equals) => Token::new(
                        TokenType::StarStar,
                        &"**".to_string(),
                        self.line,
                        index - self.last_newline,
                    ),
                    None => Token::new(
                        TokenType::Star,
                        &char.to_string(),
                        self.line,
                        index - self.last_newline,
                    ),
                },

                // Slash,
                '/' => match self.source.next_if_eq(&(index + 1, '/')) {
                    Some(_equals) => {
                        let mut temp_index: usize = 0;
                        let comment = self
                            .source
                            .by_ref()
                            .take_while(|(pos, c)| {
                                temp_index = pos.clone();
                                if c == &'\n' {
                                    self.line += 1;
                                    self.last_newline = index + pos;
                                }
                                println!(
                                    "index: {:?}, pos: {:?}, last_newline: {:?}",
                                    index, pos, self.last_newline
                                );
                                *c != '\n'
                            })
                            .map(|(_pos, c)| c)
                            .collect();

                        println!("index: {:?}, last_newline: {:?}", index, self.last_newline);
                        Token::new(
                            TokenType::Comment,
                            &comment,
                            self.line,
                            // temp_index accounts for the chars processed in take_while but not yet consumed by the iterator
                            temp_index - self.last_newline,
                        )
                    }
                    None => Token::new(
                        TokenType::Slash,
                        &char.to_string(),
                        self.line,
                        index - self.last_newline,
                    ),
                },

                // Literals
                '"' => {
                    let mut last_matched: char = '\0';

                    let s: String = self
                        .source
                        .by_ref()
                        .take_while(|(_pos, c)| {
                            last_matched = *c;
                            if c == &'\n' {
                                self.line += 1;
                                self.last_newline = index;
                            };
                            *c != '"'
                        })
                        .map(|(_pos, c)| c)
                        .collect();

                    match last_matched {
                        '"' => {
                            Token::new(TokenType::String, &s, self.line, index - self.last_newline)
                        }
                        _ => {
                            Token::new(TokenType::Invalid, &s, self.line, index - self.last_newline)
                        }
                    }
                }
                // Identifier,
                // String,
                // Number,

                // Keywords
                // And,
                // Class,
                // Else,
                // False,
                // Fun,
                // For,
                // If,
                // Nil,
                // Or,
                // Print,
                // Return,
                // Super,
                // This,
                // True,
                // Var,
                // While,

                // EOF,
                '\n' => {
                    self.last_newline = index;
                    self.line += 1;
                    Token::new(
                        TokenType::Whitespace,
                        &char.to_string(),
                        self.line,
                        index - self.last_newline,
                    )
                }
                ' ' | '\r' | '\t' => Token::new(
                    TokenType::Whitespace,
                    &char.to_string(),
                    self.line,
                    index - self.last_newline,
                ),
                unknown => {
                    RuneError::new(ErrorType::Scanner, self.line, 5, "Invalid token");
                    Token::new(
                        TokenType::Invalid,
                        &unknown.to_string(),
                        self.line,
                        index - self.last_newline,
                    )
                }
            };
            // if token.r#type == TokenType::Whitespace || token.r#type == TokenType::Comment {
            //     continue;
            // }
            self.tokens.push(token);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = "abcdefg=hi+jk".to_string();
        let mut scanner = Scanner::new(&test_input);
        scanner.scan_tokens();
        assert_eq!(4, 2 * 2);
    }
}
