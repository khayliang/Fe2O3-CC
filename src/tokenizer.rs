// TODO: Use Number token instead of Integer to support decimals
#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    Keyword(&'a str),
    Identifier(&'a str),
    Integer(&'a str),
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Semicolon,
}

pub struct TokenFactory {}
impl TokenFactory {
    fn create(token: &str) -> Token {
        if is_string_number(token) {
            return Token::Integer(token);
        } else if token.len() == 1 {
            let token_ch = token.chars().nth(0).unwrap();
            match token_ch {
                '{' => return Token::OpenBrace,
                '}' => return Token::CloseBrace,
                '(' => return Token::OpenBracket,
                ')' => return Token::CloseBracket,
                ';' => return Token::Semicolon,
                _ => panic!("Invalid symbol"),
            }
        } else {
            // match to keyword
            match token {
                "int" => return Token::Keyword(token),
                "return" => return Token::Keyword(token),
                _ => {}
            }
            // all other strings are identifiers
            return Token::Identifier(token);
        }
    }
}

fn is_string_number(s: &str) -> bool {
    s.chars().all(char::is_numeric)
}

// TODO: cleanup tokenize function
pub fn tokenize(code: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut found_token: bool = false;
    let mut token_start: usize = 0;
    for (idx, ch) in code.chars().enumerate() {
        if ch.is_whitespace() {
            if found_token {
                tokens.push(TokenFactory::create(&code[token_start..idx]));
                found_token = false;
            }
            continue;
        } else if ch.is_ascii_punctuation() {
            if found_token {
                tokens.push(TokenFactory::create(&code[token_start..idx]));
                found_token = false;
            }
            tokens.push(TokenFactory::create(&code[idx..idx + 1]));
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
    fn test_token_factory_builds_punctuation() {
        let tokens = ["{", "}", "(", ")", ";"];
        let enums = vec![
            Token::OpenBrace,
            Token::CloseBrace,
            Token::OpenBracket,
            Token::CloseBracket,
            Token::Semicolon,
        ];
        for (idx, token_str) in tokens.iter().enumerate() {
            assert_eq!(&enums[idx], &TokenFactory::create(token_str));
        }
    }

    #[test]
    fn test_token_factory_builds_integer_constants() {
        let tokens = ["2", "23"];
        let enums = vec![Token::Integer("2"), Token::Integer("23")];
        for (idx, token_str) in tokens.iter().enumerate() {
            assert_eq!(&enums[idx], &TokenFactory::create(token_str));
        }
    }
    #[test]
    fn test_token_factory_builds_identifiers() {
        let identifier_tokens = ["main", "some_name"];
        let identifier_enums = vec![Token::Identifier("main"), Token::Identifier("some_name")];
        for (idx, token_str) in identifier_tokens.iter().enumerate() {
            assert_eq!(&identifier_enums[idx], &TokenFactory::create(token_str));
        }
    }
    #[test]
    fn test_token_factory_builds_keywords() {
        let keyword_tokens = ["int", "return"];
        let keyword_enums = vec![Token::Keyword("int"), Token::Keyword("return")];
        for (idx, token_str) in keyword_tokens.iter().enumerate() {
            assert_eq!(&keyword_enums[idx], &TokenFactory::create(token_str));
        }
    }
    #[test]
    fn test_token_builder_builds_return_2() {
        let tokens = ["int", "main", "(", ")", "{", "return", "2", ";", "}"];
        let correct_tokens = vec![
            Token::Keyword("int"),
            Token::Identifier("main"),
            Token::OpenBracket,
            Token::CloseBracket,
            Token::OpenBrace,
            Token::Keyword("return"),
            Token::Integer("2"),
            Token::Semicolon,
            Token::CloseBrace,
        ];
        for (idx, token_str) in tokens.iter().enumerate() {
            assert_eq!(&correct_tokens[idx], &TokenFactory::create(token_str));
        }
    }

    #[test]
    fn test_tokenize_empty_string() {
        let code = "";
        let correct_tokens: Vec<Token> = vec![];
        let parsed_tokens = tokenize(code);
        assert_eq!(correct_tokens.len(), parsed_tokens.len());
    }

    #[test]
    fn test_tokenize_func_tokenizes_string() {
        let code = "
            int main() {
                return 2;
            }
        ";
        let correct_tokens = vec![
            Token::Keyword("int"),
            Token::Identifier("main"),
            Token::OpenBracket,
            Token::CloseBracket,
            Token::OpenBrace,
            Token::Keyword("return"),
            Token::Integer("2"),
            Token::Semicolon,
            Token::CloseBrace,
        ];
        let parsed_tokens = tokenize(code);
        for (idx, token) in parsed_tokens.iter().enumerate() {
            assert_eq!(&correct_tokens[idx], token);
        }
    }
}
