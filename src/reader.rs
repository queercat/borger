use crate::tokenizer::{tokenize, BorgerToken};
use std::io::{stdin, Read, stdout, Write};

pub fn repl() {
    loop {
        let tokens = eval(read());

        println!("{:?}", tokens);
    }
}

pub fn eval(source: String) -> Box<Vec<BorgerToken>> {
    tokenize(source.as_str())
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
