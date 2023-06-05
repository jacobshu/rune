mod Token;
mod error;

pub struct Scanner {
    source: Iterator,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(input: String) -> Self {
        Self {
            source: input.char_indices().peekable(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&mut self) {
        self.current >= source.len()
    }

    fn scan_tokens(&mut self) {
        while (!self.is_at_end()) {
            self.start = self.current;
            self.scan_token()
        }
    }

    fn scan_token(&mut self) {
        while let Some((index, char)) = self.source.next() {
        let token = match char {
            '(' => Token::new(TokenType::LeftParen), 
            ')' => Token::new(TokenType::RightParen),
            '{' => Token::new(TokenType::LeftBrace),
            '}' => Token::new(TokenType::RightBrace),
            ',' => Token::new(TokenType::Comma),
            '.' => Token::new(TokenType::Dot),
            '+' => Token::new(TokenType::Plus),
            '-' => Token::new(TokenType::Minus),
            ';' => Token::new(TokenType::Semicolon),

        // // One to two characters
        '=' => {
            match self.source.next_if_eq(&(index + 1, '=')) {
            	Some(_equals) => Token::new(TokenType::EqualEqual),
            	None => Token::new(TokenType::Equal),
            }
        },
        '!' => {
            match self.source.next_if_eq(&(index + 1, '=')) {
            	Some(_equals) => Token::new(TokenType::BangEqual),
            	None => Token::new(TokenType::Bang)
            }
        },
        '>' => {
            match self.source.next_if_eq(&(index + 1, '=')) {
                Some(_equals) => Token::new(TokenType::GreaterEqual),
                None => Token::new(TokenType::Greater)
            }
        },
        '<' => {
            match self.source.next_if_eq(&(index + 1, '=')) {
                Some(_equals) => Token::new(TokenType::LessEqual),
                None => Token::new(TokenType::Less)
            }
        },
        '*' => {
            match self.source.next_if_eq(&(index + 1, '*')) {
                Some(_equals) => Token::new(TokenType::StarStar),
                None => Token::new(TokenType::Star)
            }
        },
                
        // Slash,

        // // Literals
        // Identifier,
        // String,
        // Number,

        // // Keywords
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
        _ => Token::Invalid(format!("{}", ch)),
        };
        tokens.push(token);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = String::new("abcdefg=hi+jk");
        let scanner = Scanner::new(test_input);
        scanner.scan();
        assert_eq!(4, 2 * 2);
    }
}
