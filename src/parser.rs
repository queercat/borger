use crate::tokenizer::{match_alpha, match_numeric, match_symbol, BorgerToken};
use std::iter::Peekable;

#[derive(Debug, PartialEq, Clone)]
pub enum BorgerType {
    Symbol(String),
    Number(f64),
    List(Vec<BorgerType>),
    Boolean(bool),
    Function(fn(BorgerType) -> BorgerType),
    Null,
}

pub fn read_atom(token: &BorgerToken) -> BorgerType {
    let c = token.text.chars().nth(0).unwrap();
    let mut ast = BorgerType::Null;

    if token.text == "true" || token.text == "false" {
        ast = BorgerType::Boolean(token.text == "true");
    } else if token.text == "null" {
        ast = BorgerType::Null
    } else if match_alpha(c) || match_symbol(c) {
        ast = BorgerType::Symbol(token.text.clone());
    } else if match_numeric(c) {
        ast = BorgerType::Number(token.text.parse::<f64>().unwrap())
    }

    ast
}

pub fn read_list<'a, I>(tokens: &mut Peekable<I>) -> BorgerType
where
    I: Iterator<Item = &'a BorgerToken>,
{
    let mut token = *tokens.peek().unwrap();
    let mut list = Vec::new();

    while token.text.as_str() != ")" {
        list.push(read_form(tokens));
        token = tokens.peek().expect("Expected ) but found None instead")
    }

    tokens.next();

    BorgerType::List(list)
}

pub fn read_form<'a, I>(tokens: &mut Peekable<I>) -> BorgerType
where
    I: Iterator<Item = &'a BorgerToken>,
{
    let token = tokens.next().unwrap();

    let ast = match token.text.as_str() {
        "(" => read_list(tokens),
        _ => read_atom(&token),
    };

    ast
}
