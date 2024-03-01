use crate::reader::repl;

pub mod reader;
mod test;
pub mod tokenizer;
mod parser;

fn main() {
    repl();
}
