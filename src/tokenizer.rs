pub fn tokenize(code: &str) -> Vec<&str> {
    let mut tokens: Vec<&str> = Vec::new();

    let mut found_token: bool = false;
    let mut token_start: usize = 0;
    for (idx, ch) in code.chars().enumerate() {
        if ch.is_whitespace() {
            if found_token {
                tokens.push(&code[token_start..idx]);
                found_token = false;
            }
            continue;
        } else if ch.is_ascii_punctuation() {
            if found_token {
                tokens.push(&code[token_start..idx]);
                found_token = false;
            }
            tokens.push(&code[idx..idx + 1]);
            continue;
        }

        if !found_token {
            token_start = idx;
            found_token = true;
        }
    }
    return tokens;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_empty_string() {
        let code = "";
        let correct_tokens: Vec<Tokens> = vec![];
        let parsed_tokens = tokenize(code);
        assert_eq!(correct_tokens, parsed_tokens);
    }

    #[test]
    fn test_tokenize_func_tokenizes_string() {
        let code = "
            int main() {
                return 2;
            }
        ";
        let correct_tokens = vec![
            Tokens::Keyword("int"), 
            Tokens::Identifier("main"), 
            Tokens::OpenBracket, 
            Tokens::CloseBracket, 
            Tokens::OpenBrace, 
            Tokens::Keyword("return"), 
            Tokens::Integer(2),
            Tokens::Semicolon,
            Tokens::CloseBrace
        ]
        let parsed_tokens = tokenize(code);
        assert_eq!(correct_tokens, parsed_tokens);
    }
}
