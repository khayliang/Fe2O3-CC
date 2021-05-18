pub fn tokenize(code: &str) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_func_tokenizes_string() {
        let code = "
            int main() {
                return 2;
            }
        ";
        let correct_tokens = vec!["int", "main", "(", ")", "{", "return", "2", ";", "}"];
        let parsed_tokens = tokenize(code);
        assert_eq!(correct_tokens, parsed_tokens);
    }
}
