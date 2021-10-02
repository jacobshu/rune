#[derive(Debug)]
pub enum ErrorType {
  Scanner,
  Parser,
  Syntax
}

pub struct Error {
  pub r#type: ErrorType,
  pub message: String,
  pub line: u32,
  pub column: u32,
}

impl Error {
  fn new(err_type: ErrorType, line: i32, col: i32, message: &str) {
    Error::report(line, col, err_type, message);
  }

  fn report(line: i32, col: i32, err_type: ErrorType, message: &str) {
    eprintln!(
      "[line {}, column {}] Error ({:?}): {}",
      line, col, err_type, message
    );
  }
}
