use crate::reader::repl;

pub mod reader;
mod test;
pub mod tokenizer;

fn main() {
    repl();
}
