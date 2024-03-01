use std::any::Any;
use crate::tokenizer::{match_alpha, match_numeric, tokenize, BorgerToken};
use std::fmt::Pointer;
use std::io::{stdin, stdout, Write};
use std::iter::Peekable;

#[derive(Debug)]
enum BorgerType {
    Symbol(String),
    Number(f64),
    List(Vec<BorgerType>),
    Boolean(bool),
    Null,
}

pub fn repl() {
    loop {
        let ast = eval(read());

        println!("{:?}", ast);
    }
}

fn read_atom(token: &BorgerToken) -> BorgerType {
    let c = token.text.chars().nth(0).unwrap();
    let mut ast = BorgerType::Null;

    if token.text == "true" || token.text == "false" {
        ast = BorgerType::Boolean(token.text == "true");
    } else if token.text == "null" {
        ast = BorgerType::Null
    } else if match_alpha(c) {
        ast = BorgerType::Symbol(token.text.clone());
    } else if match_numeric(c) {
        ast = BorgerType::Number(token.text.parse::<f64>().unwrap())
    }

    ast
}

fn read_list<'a, I>(tokens: &mut Peekable<I>) -> BorgerType
where I: Iterator<Item = &'a BorgerToken> {
    let mut token = *tokens.peek().unwrap();
    let mut list = Vec::new();

    while token.text.as_str() != ")" {
        list.push(read_form(tokens));
        token = tokens.next().unwrap();
    }

    BorgerType::List(list)
}

fn read_form<'a, I>(tokens: &mut Peekable<I>) -> BorgerType
where I: Iterator<Item = &'a BorgerToken> {
    let token = tokens.next().unwrap();

    let ast = match token.text.as_str() {
        "(" => read_list(tokens),
        _ => read_atom(&token),
    };

    ast
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
