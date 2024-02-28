#[cfg(test)]
mod tests {
    use crate::tokenizer::{tokenize, BorgerToken};

    #[test]
    fn tokenize_when_given_correct_source_should_produce_valid_borkens() {
        let expected = vec![
            BorgerToken {
                text: String::from("hello"),
            },
            BorgerToken {
                text: String::from("world"),
            },
        ];

        let borkens = tokenize("hello world");

        assert_eq!(*borkens, expected);
    }

    #[test]
    fn tokenize_when_given_valid_list_should_produce_valid_borkens() {
        let expected = vec![
            BorgerToken {
                text: String::from("("),
            },
            BorgerToken {
                text: String::from("("),
            },
            BorgerToken {
                text: String::from("("),
            },
            BorgerToken {
                text: String::from("+"),
            },
            BorgerToken {
                text: String::from("3"),
            },
            BorgerToken {
                text: String::from("4"),
            },
            BorgerToken {
                text: String::from(")"),
            },
            BorgerToken {
                text: String::from(")"),
            },
            BorgerToken {
                text: String::from(")"),
            },
        ];

        let borkens = tokenize("(((+ 3 4 )))");

        assert_eq!(*borkens, expected);
    }

    #[test]
    #[should_panic]
    fn tokenize_when_given_invalid_source_should_panic() {
        tokenize("🥺");
    }
}
