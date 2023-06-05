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
            source: input.chars().peekable(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_tokens(&mut self) {
        while (!self.is_at_end()) {
            self.start = self.current;
            scan_token()
        }
    }

    fn is_at_end(&mut self) {
        self.current >= source.len()
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
