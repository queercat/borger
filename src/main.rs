use std::io::{stdin, stdout, Write};

use parser::{read_form, BorgerType};
use tokenizer::tokenize;

mod parser;
mod test;

pub mod eval;
pub mod tokenizer;

fn main() {
    loop {
        let ast = eval(read());

        println!("{:?}", ast);
    }
}

fn eval(source: String) -> BorgerType {
    let binding = tokenize(source.as_str());
    let mut tokens = binding.iter().peekable();

    let ast = read_form(&mut tokens);

    ast
}

pub fn read() -> String {
    print!("> ");
    let result = stdout().flush();

    if result.is_err() {
        panic!("Unable to flush STDOUT.");
    }

    let mut input = String::new();
    let result = stdin().read_line(&mut input);

    if result.is_err() {
        panic!("Unable to read line for REPL.");
    }

    input
}
