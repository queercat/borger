use crate::reader::repl;
use crate::tokenizer::tokenize;
pub mod reader;
mod test;
pub mod tokenizer;

fn main() {
    repl();
}
