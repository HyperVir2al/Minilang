mod ast;
mod evaluator;
mod lexer;
mod parser;
mod value;

use std::env;
use std::fs;
use std::process;

use evaluator::Evaluator;
use lexer::Lexer;
use parser::Parser;

fn run_source(source: &str) -> Result<(), String> {
    let tokens = Lexer::new(source).tokenize()?;
    let program = Parser::new(tokens).parse_program()?;
    Evaluator::new().run(&program)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Использование: minilang <файл.ml>");
        process::exit(1);
    }
    let source = match fs::read_to_string(&args[1]) {
        Ok(text) => text,
        Err(err) => {
            eprintln!("Не удалось прочитать файл '{}': {}", args[1], err);
            process::exit(1);
        }
    };
    if let Err(message) = run_source(&source) {
        eprintln!("{}", message);
        process::exit(1);
    }
}
