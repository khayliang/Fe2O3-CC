#[derive(Debug, PartialEq, Clone)]

pub enum Token {
    Keyword(String),
    Identifier(String),
    Integer(String),
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Semicolon,
}

pub struct TokenFactory {}
impl TokenFactory {
    fn create(token: &str) -> Token {
        let token_string = token.to_string();

        if is_string_number(&token_string) {
            return Token::Integer(token_string);
        } else if token_string.len() == 1 {
            let token_ch = token_string.chars().nth(0).unwrap();
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
                "int" => return Token::Keyword(token_string),
                "return" => return Token::Keyword(token_string),
                _ => {}
            }
            // all other strings are identifiers
            return Token::Identifier(token_string);
        }
    }
}

fn is_string_number(s: &String) -> bool {
    s.chars().all(char::is_numeric)
}

fn string_to_number(s: &String) -> i32 {
    let mut error_str = "Invalid_syntax: ".to_string();
    error_str.push_str(s);
    s.parse::<i32>().expect(&error_str)
}

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
        let enums = vec![
            Token::Integer("2".to_string()),
            Token::Integer("23".to_string()),
        ];
        for (idx, token_str) in tokens.iter().enumerate() {
            assert_eq!(&enums[idx], &TokenFactory::create(token_str));
        }
    }
    #[test]
    fn test_token_factory_builds_identifiers() {
        let identifier_tokens = ["main", "some_name"];
        let identifier_enums = vec![
            Token::Identifier("main".to_string()),
            Token::Identifier("some_name".to_string()),
        ];
        for (idx, token_str) in identifier_tokens.iter().enumerate() {
            assert_eq!(&identifier_enums[idx], &TokenFactory::create(token_str));
        }
    }
    #[test]
    fn test_token_factory_builds_keywords() {
        let keyword_tokens = ["int", "return"];
        let keyword_enums = vec![
            Token::Keyword("int".to_string()),
            Token::Keyword("return".to_string()),
        ];
        for (idx, token_str) in keyword_tokens.iter().enumerate() {
            assert_eq!(&keyword_enums[idx], &TokenFactory::create(token_str));
        }
    }
    #[test]
    fn test_token_builder_builds_return_2() {
        let tokens = ["int", "main", "(", ")", "{", "return", "2", ";", "}"];
        let correct_tokens = vec![
            Token::Keyword("int".to_string()),
            Token::Identifier("main".to_string()),
            Token::OpenBracket,
            Token::CloseBracket,
            Token::OpenBrace,
            Token::Keyword("return".to_string()),
            Token::Integer("2".to_string()),
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
            Token::Keyword("int".to_string()),
            Token::Identifier("main".to_string()),
            Token::OpenBracket,
            Token::CloseBracket,
            Token::OpenBrace,
            Token::Keyword("return".to_string()),
            Token::Integer("2".to_string()),
            Token::Semicolon,
            Token::CloseBrace,
        ];
        let parsed_tokens = tokenize(code);
        for (idx, token) in parsed_tokens.iter().enumerate() {
            assert_eq!(&correct_tokens[idx], token);
        }
    }
}
