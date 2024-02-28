#[derive(Debug)]
pub struct Token {
    pub text: String
}

pub fn tokenize(source: &str) -> Box<Vec<Token>> {
    let mut tokens: Box<Vec<Token>> = Box::new(Vec::new());
    let mut chars = source.chars().peekable();

    loop {
        if chars.peek() == None { break; }

        let mut char = chars.next().unwrap();
        let mut text = String::new();

        if match_whitespace(char) {
            loop {
                if chars.peek() == None { return tokens; }

                let next = *chars.peek().unwrap();
                char = chars.next().unwrap();

                if !match_whitespace(next) { break; }
            }
        }

        if match_alphanumeric(char) {
            loop {
                text.push(char);

                if chars.peek() == None { break; }
                if !match_alphanumeric(*chars.peek().unwrap()) { break; }

                char = chars.next().unwrap();
            }
        }

        tokens.push(Token { text })
    }

    return tokens;
}

fn match_alphanumeric(char: char) -> bool {
   (char >= '0' && char <= '9') || (char >= 'a' && char <= 'z') || (char >= 'A' && char <= 'Z')
}

fn match_whitespace(char: char) -> bool {
    char == ' ' || char == '\n' || char == '\t'
}