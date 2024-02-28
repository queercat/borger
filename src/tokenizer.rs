#[derive(Debug)]
pub struct Token<'a> {
    pub text: &'a str
}

pub fn tokenize(source: &str) -> Box<Vec<Token>> {
    let mut tokens: Box<Vec<Token>> = Box::new(Vec::new());

    source.split_whitespace().for_each(|t| tokens.push(Token {text: t }));

    return tokens;
}