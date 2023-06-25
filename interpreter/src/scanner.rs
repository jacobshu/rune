// use crate::error::{ErrorType, RuneError};
use crate::token::{Token, TokenType};
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::CharIndices;

pub struct Scanner<'a> {
    source: Peekable<CharIndices<'a>>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    last_newline: usize,
    keywords: HashMap<String, TokenType>,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            source: input.char_indices().peekable(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            last_newline: 0,
            keywords: HashMap::from([
                ("and".to_string(), TokenType::And),
                ("class".to_string(), TokenType::Class),
                ("else".to_string(), TokenType::Else),
                ("false".to_string(), TokenType::False),
                ("fn".to_string(), TokenType::Fn),
                ("for".to_string(), TokenType::For),
                ("if".to_string(), TokenType::If),
                ("nil".to_string(), TokenType::Nil),
                ("or".to_string(), TokenType::Or),
                ("print".to_string(), TokenType::Print),
                ("return".to_string(), TokenType::Return),
                ("super".to_string(), TokenType::Super),
                ("this".to_string(), TokenType::This),
                ("true".to_string(), TokenType::True),
                ("var".to_string(), TokenType::Var),
                ("while".to_string(), TokenType::While)
            ]),

        }
    }

    fn is_at_end(&mut self) -> bool {
        self.source.peek().is_none()
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.scan_token()
        }
        self.tokens.clone()
    }
    
    fn get_keyword(&mut self, identifier_or_keyword: String, index: usize) -> Token {
       let token_type: Option<&TokenType> = self.keywords.get(&identifier_or_keyword);
       match token_type {
           Some(token_type) => Token::new(token_type.to_owned(), &identifier_or_keyword, self.line, index - self.last_newline),
           None => Token::new(TokenType::Identifier, &identifier_or_keyword, self.line, index - self.last_newline)
       }
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
                                temp_index = *pos;
                                if c == &'\n' {
                                    self.line += 1;
                                    self.last_newline = index + pos;
                                }
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
                // String
                '"' => {
                    let mut last_matched: char = '\0';
                    let mut temp_index: usize = 0;

                    let s: String = self
                        .source
                        .by_ref()
                        .take_while(|(pos, c)| {
                            last_matched = *c;
                            temp_index = *pos;
                            if c == &'\n' {
                                self.line += 1;
                                self.last_newline = *pos;
                            };
                            // println!("pos: {:?},  c: {:?}, last_n: {:?}", pos, c, self.last_newline);
                            c != &'"'
                        })
                        .map(|(_pos, c)| c)
                        .collect();

                    match last_matched {
                        '"' => {
                            Token::new(TokenType::String, &s, self.line, temp_index - self.last_newline)
                        }
                        _ => {
                            Token::new(TokenType::Invalid, &s, self.line, temp_index - self.last_newline)
                        }
                    }
                }
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
//                unknown => {
//                    RuneError::new(ErrorType::Scanner, self.line, 5, "Invalid token");
//                    Token::new(
//                        TokenType::Invalid,
//                        &unknown.to_string(),
//                        self.line,
//                        index - self.last_newline,
//                    )
//                }
            

                // Numbers, keywords, & identifiers
                alphanumeric => {
                    match alphanumeric.is_ascii_digit() {
                      true => {
                        let mut number = Vec::new();
                        number.push(alphanumeric);
                        
                        let mut dots: u8 = 0;
                        let mut remaining: Vec<char> = self.source.by_ref().take_while(|(_pos, n)| { 
                            if n == &'.' && dots == 0 {
                                dots += 1;
                                true
                            } else {
                                n.is_ascii_digit()
                            }}).map(|(_pos, c)| c).collect();
                      
                        number.append(&mut remaining);
                        let number_string = number.iter().collect();
                        Token::new(TokenType::Number, &number_string, self.line, index - self.last_newline)
                    }
                       false => {
                           let mut alpha = Vec::new();
                           alpha.push(alphanumeric);
                           let mut remaining: Vec<char> = self.source.by_ref().take_while(|(_pos, s)| {
                                s.is_ascii_alphabetic()
                           }).map(|(_pos,c)| c).collect();
                           
                           alpha.append(&mut remaining);
                           let alpha_string = alpha.iter().collect();
                           
                           self.get_keyword(alpha_string, index)
                           //Token::new(TokenType::Nil, &alpha_string, self.line, index - self.last_newline)

                       }
                   }
                }

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

 };

            if token.r#type == TokenType::Whitespace || token.r#type == TokenType::Comment {
                continue;
            }
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
