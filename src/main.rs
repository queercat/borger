use crate::tokenizer::{Token, tokenize};
pub mod tokenizer;

fn main() {
    let tokens = tokenize("hello world");

    println!("{:?}", *tokens)
}