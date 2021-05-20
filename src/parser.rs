use crate::syntax::*;
use crate::tokenizer::Token;

pub struct TokenIteratorContainer<'a> {
    iterator: std::iter::Peekable<std::slice::Iter<'a, Token<'a>>>,
}

impl<'a> TokenIteratorContainer<'a> {
    pub fn new(token: &'a Vec<Token>) -> TokenIteratorContainer<'a> {
        let iterator = token.iter().peekable();
        TokenIteratorContainer { iterator }
    }
    fn get_iter(&mut self) -> &mut std::iter::Peekable<std::slice::Iter<'a, Token<'a>>> {
        &mut self.iterator
    }
}

fn string_to_number(s: &str) -> i32 {
    let mut error_str = "Invalid_syntax: ".to_string();
    error_str.push_str(s);
    s.parse::<i32>().expect(&error_str)
}

fn parse_keyword_to_variable_type(token: &Token) -> Result<Type, String> {
    match token {
        Token::Keyword(keyword) => match *keyword {
            "int" => return Ok(Type::Integer(0)),
            _ => return Err(format!("Invalid syntax: {}", keyword)),
        },
        _ => {
            let mut msg = String::from("Invalid syntax: ");
            msg.push_str(&format!("{:?}", token));
            return Err(msg);
        }
    }
}

fn parse_expression(iterator: &mut TokenIteratorContainer) -> Result<Box<dyn Expression>, String> {
    let mut tokens_iter = iterator.get_iter();
    // TODO: refactor this to be more idiomatic and rusty
    while let Some(token) = tokens_iter.next() {
        match token {
            Token::Integer(val) => {
                let int_variable = Type::Integer(string_to_number(&val));
                let next_token = *tokens_iter.peek().unwrap();
                if *next_token == Token::Semicolon {
                    return Ok(Box::new(expressions::Constant::new(int_variable)));
                }
            }
            _ => {
                let mut msg = String::from("Unidentified expression");
                msg.push_str(format!("{:?}", token).as_str());
                return Err(msg);
            }
        }
    }
    return Err(String::from("Something went wrong"));
}

fn parse_statement(iterator: &mut TokenIteratorContainer) -> Result<Box<dyn Statement>, String> {
    let tokens_iter = iterator.get_iter();
    // TODO: refactor this to be more idiomatic and rusty
    while let Some(token) = tokens_iter.next() {
        match token {
            Token::Keyword(keyword) => match *keyword {
                "return" => {
                    let expression = match parse_expression(iterator) {
                        Ok(expression) => expression,
                        Err(msg) => return Err(msg),
                    };
                    let return_statement = Box::new(statements::Return::new(expression));
                    return Ok(return_statement);
                }
                "function" => {
                    let return_type = *tokens_iter.peek().unwrap();
                }
                _ => {
                    let mut msg = String::from("Invalid syntax: ");
                    msg.push_str(keyword);
                    return Err(msg);
                }
            },
            _ => {
                let mut msg = String::from("Invalid syntax: ");
                msg.push_str(&format!("{:?}", token));
                return Err(msg);
            }
        }
    }
    return Err(String::from("Something went wrong"));
}

/*
fn parse(tokens: Vec<Token>) -> Program {

}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_iterator_container() {
        let tokens: Vec<Token> = vec![Token::Integer("2"), Token::Semicolon];
        let mut token_iterator = TokenIteratorContainer::new(&tokens);
        let iter = token_iterator.get_iter();
        assert_eq!(&tokens[0], iter.next().unwrap());
        assert_eq!(&tokens[1], iter.next().unwrap());
    }

    #[test]
    fn test_parse_keyword_token_to_variable_type() {
        let tokens: Vec<Token> = vec![Token::Keyword("int")];

        let correct_types: Vec<Type> = vec![Type::Integer(0)];

        for (idx, token) in tokens.iter().enumerate() {
            let variable_type: Type = parse_keyword_to_variable_type(&token).unwrap();
            let correct_format = format!("{}", correct_types[idx]);
            let test_format = format!("{}", variable_type);
            assert_eq!(correct_format, test_format);
        }
    }

    #[test]
    fn test_parse_expression_tokens() {
        let tokens: Vec<Token> = vec![Token::Integer("2"), Token::Semicolon];
        let mut token_iterator = TokenIteratorContainer::new(&tokens);
        let expression: Box<dyn Expression> = match parse_expression(&mut token_iterator) {
            Ok(val) => val,
            Err(msg) => panic!("{}", msg),
        };
        let correct_expression = test_utils::create_test_constant_expression();
        let correct_format = format!("{}", correct_expression);
        let test_format = format!("{}", expression);
        assert_eq!(correct_format, test_format);
    }

    #[test]
    fn test_parse_return_statement_tokens() {
        let tokens: Vec<Token> = vec![
            Token::Keyword("return"),
            Token::Integer("2"),
            Token::Semicolon,
        ];
        let mut token_iterator = TokenIteratorContainer::new(&tokens);
        let return_statement: Box<dyn Statement> = match parse_statement(&mut token_iterator) {
            Ok(val) => val,
            Err(msg) => panic!("{}", msg),
        };
        let correct_expression = test_utils::create_test_return_statement();
        let correct_format = format!("{}", correct_expression);
        let test_format = format!("{}", return_statement);
        assert_eq!(correct_format, test_format);
    }

    fn test_parse_function__statement_tokens() {
        let tokens: Vec<Token> = vec![
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
        let mut token_iterator = TokenIteratorContainer::new(&tokens);
        let function_node: Box<dyn Statement> = match parse_statement(&mut token_iterator) {
            Ok(val) => val,
            Err(msg) => panic!("{}", msg),
        };
        let correct_expression = test_utils::create_test_function();
        let correct_format = format!("{}", correct_expression);
        let test_format = format!("{}", function_node);
        assert_eq!(correct_format, test_format);
    }

    #[test]
    fn test_parse_all_tokens_into_program_with_main() {
        let tokens: Vec<Token> = vec![
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
        /*
        let program: Program = super::parse_program(tokens);
        let main_function: Function = program.root;
        assert_eq!(main_function.name, String::from("main"));
        assert!(std::mem::discriminant(&main_function.return_type) == std::mem::discriminant(&Type::Integer(0)));
        */
    }
}
