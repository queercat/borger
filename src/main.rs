use crate::tokenizer::tokenize;
pub mod tokenizer;

fn main() {
    let tokens = tokenize("hello world 123   \n 456");

    println!("{:?}", *tokens)
}