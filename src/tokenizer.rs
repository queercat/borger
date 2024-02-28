#[derive(Debug)]
pub struct BorgerToken {
    pub text: String
}

impl PartialEq for BorgerToken {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

/// Produces a vector of Borger tokens from a given source program.
pub fn tokenize(source: &str) -> Box<Vec<BorgerToken>> {
    let mut tokens: Box<Vec<BorgerToken>> = Box::new(Vec::new());
    let mut chars = source.chars().peekable();

    let mut line_number = 0;
    let mut column_number = 0;

    loop {
        if chars.peek() == None { break; }

        let mut char = chars.next().unwrap();
        let mut text = String::new();

        if match_whitespace(char) {
            loop {
                if chars.peek() == None { return tokens; }

                if char == '\n' {
                    line_number += 1;
                }

                if char == '\t' || char == ' ' {
                    column_number += 1;
                }

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
                column_number += 1;
            }
        }

        else {
            panic!("Invalid token found while lexing! {char} <-- at line {line_number} col {column_number}")
        }

        tokens.push(BorgerToken { text })
    }

    return tokens;
}

fn match_alphanumeric(char: char) -> bool {
   (char >= '0' && char <= '9') || (char >= 'a' && char <= 'z') || (char >= 'A' && char <= 'Z')
}

fn match_whitespace(char: char) -> bool {
    char == ' ' || char == '\n' || char == '\t'
}