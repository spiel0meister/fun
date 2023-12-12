use std::io::{Error, ErrorKind};

use std::env::args;
use std::fs::read_to_string;

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
    println!("{}", content);
    Ok(())
}
