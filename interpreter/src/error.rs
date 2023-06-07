#[derive(Debug)]
pub enum ErrorType {
    Scanner,
    Parser,
    Syntax,
}

pub struct RuneError {
    pub r#type: ErrorType,
    pub message: String,
    pub line: u32,
    pub column: u32,
}

impl RuneError {
    pub fn new(err_type: ErrorType, line: usize, col: usize, message: &str) {
        RuneError::report(line, col, err_type, message);
    }

    fn report(line: usize, col: usize, err_type: ErrorType, message: &str) {
        eprintln!(
            "[line {}, column {}] Error ({:?}): {}",
            line, col, err_type, message
        );
    }
}
