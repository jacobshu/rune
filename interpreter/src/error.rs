#[derive(Debug)]
pub enum ErrorType {
    Scanner,
    Parser,
    Syntax,
}

pub struct RuneError {
    pub r#type: ErrorType,
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl RuneError {
    pub fn new(err_type: ErrorType, line: usize, col: usize, message: &str) -> Self {
        RuneError {
            line, 
            column: col, 
            r#type: err_type, 
            message: message.to_string(),
        }
    }

    fn report(self) {
        eprintln!(
            "[line {}, column {}] Error ({:?}): {}",
            self.line, self.column, self.r#type, self.message
        );
    }
}
