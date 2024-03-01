use crate::tokenizer::tokenize;
use crate::parser::{BorgerType, read_form};
use std::io::{stdin, stdout, Write};

pub fn repl() {
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
