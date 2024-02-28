use crate::tokenizer::tokenize;
pub mod tokenizer;

fn main() {
    let tokens = tokenize("hello world");

    println!("{:?}", *tokens)
}