use crate::tokenizer::tokenize;
mod test;
pub mod tokenizer;

fn main() {
    let borkens = tokenize("(+ 3 (- 4 2))");
}
