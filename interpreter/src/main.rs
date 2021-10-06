#![allow(dead_code)]
use std::fs::read_to_string;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::exit;


mod error;
mod token;

struct Rune {
  had_error: bool,
  errors: Vec<error::Error>,
}

impl Rune {
  fn new() -> Self {
    Rune {
      had_error: false,
      errors: vec![],
    }
  }

  fn run_file(&mut self, path: &String) {
    let path = Path::new(&path);
    let buffer = read_to_string(&path).expect("Couldn't read file");
    self.run(buffer);
  }

  fn run_prompt(&mut self) {
    println!("\n‹◊◊ Rune ◊◊›\nMythic power at your fingertips\n");
    loop {
      let stdin = io::stdin();
      for line in stdin.lock().lines() {
        match line.unwrap().as_str() {
          "exit" => {
            println!("\n");
            exit(0);
          }
          "" => {}
          _ => {
            println!("Invalid input");
          }
        }
        print!("» ");
        io::stdout().flush().unwrap();
      }
    }
  }

  fn run(&mut self, source: String) {

    if self.had_error {
      println!("Errors: {}", self.errors.len())
    }

    println!("source: {}", source);
  }
}

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let mut rune = Rune::new();

  match args.len() {
    1 => {
      rune.run_prompt();
    }
    2 => {
      rune.run_file(&args[1]);
    }
    _ => {
      println!("\nUsage: rune [file]\n");
      exit(64);
    }
  }
}
