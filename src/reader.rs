use crate::tokenizer::{tokenize, BorgerToken};
use std::io::{stdin, stdout, Write};

#[derive(Debug)]
enum BorgerType {
    Symbol(String),
    Number(f64),
    List(Vec<BorgerType>)
}

pub fn repl() {
    loop {
        let ast = eval(read());

        println!("{:?}", ast);
    }
}

fn read_form(tokens) -> BorgerType {
    BorgerType::Number(1f64)
}

fn eval(source: String) -> BorgerType {
    let mut tokens = tokenize(source.as_str());

    read_form(&mut tokens)
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
