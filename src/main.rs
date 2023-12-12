use std::io::{Error, ErrorKind};

use std::env::args;
use std::fs::read_to_string;

mod interpreter;
mod tokenizer;

fn get_file_path() -> std::io::Result<String> {
    let args_vec: Vec<String> = args().collect();

    if args_vec.len() < 2 {
        return Err(Error::new(ErrorKind::InvalidInput, "Not enough arguments"));
    }

    Ok(args_vec[1].clone())
}

fn main() -> std::io::Result<()> {
    let filepath = get_file_path()?;
    let content = read_to_string(filepath)?;

    let mut main_tokenizer = tokenizer::Tokenizer::new(content);
    let tokens = main_tokenizer.tokenize()?;
    let mut main_interpreter = interpreter::Interpreter::new(tokens.clone());

    main_interpreter.interpret()?;
    Ok(())
}
