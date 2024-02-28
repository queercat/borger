#[cfg(test)]
mod tests {
    use crate::tokenizer::{BorgerToken, tokenize};

    #[test]
    fn tokenize_when_given_correct_source_should_produce_valid_borkens() {
        let expected = vec![BorgerToken{text: String::from("hello")}, BorgerToken{text: String::from("world")}];

        let borkens = tokenize("hello world");

        assert_eq!(*borkens, expected);
    }

    #[test]
    #[should_panic]
    fn tokenize_when_given_invalid_source_should_panic() {
        tokenize("🥺");
    }
}