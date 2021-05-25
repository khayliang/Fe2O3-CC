use crate::syntax::*;
use crate::tokenizer::Token;

/*
pub struct TokenIteratorContainer<'a> {
    iterator: &'a std::iter::Peekable<std::slice::Iter<'a, Token<'a>>>,
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
*/
type TokenIterator<'a> = std::iter::Peekable<std::vec::IntoIter<Token<'a>>>;

fn string_to_number(s: &str) -> i32 {
    let mut error_str = "Invalid_syntax: ".to_string();
    error_str.push_str(s);
    s.parse::<i32>().expect(&error_str)
}

fn parse_expression(tokens_iter: &mut TokenIterator) -> Result<Box<dyn Expression>, String> {
    // TODO: refactor this to be more idiomatic and rusty
    while let Some(token) = tokens_iter.next() {
        match token {
            Token::Integer(val) => {
                let int_variable = Type::Integer(string_to_number(&val));
                let next_token = tokens_iter.next().unwrap();
                match next_token {
                    Token::Semicolon => {
                        return Ok(Box::new(expressions::Constant::new(int_variable)))
                    }
                    _ => return Err(String::from("Missing semicolon")),
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

fn parse_statement(tokens_iter: &mut TokenIterator) -> Result<Statement, String> {
    // TODO: refactor this to be more idiomatic and rusty
    while let Some(token) = tokens_iter.next() {
        match token {
            Token::Keyword(keyword) => match keyword {
                "return" => {
                    let expression = match parse_expression(tokens_iter) {
                        Ok(expression) => expression,
                        Err(msg) => return Err(msg),
                    };
                    let return_statement = statements::Return::new(expression);
                    return Ok(return_statement);
                }
                "int" => {
                    let statement_type = Type::Integer(0);
                    let function_name = tokens_iter.next().unwrap();
                    let function_name = match function_name {
                        Token::Identifier(name) => name,
                        _ => return Err(format!("Invalid syntax {:?}", function_name)),
                    };
                    if !matches!(tokens_iter.next().unwrap(), Token::OpenBracket)
                        || !matches!(tokens_iter.next().unwrap(), Token::CloseBracket)
                    {
                        return Err(format!(
                            "Missing function parameters for function {}",
                            function_name
                        ));
                    }
                    if !matches!(tokens_iter.next().unwrap(), Token::OpenBrace) {
                        return Err(format!(
                            "Missing function body for function {}",
                            function_name
                        ));
                    }

                    let mut body: Vec<Statement> = vec![];
                    while !matches!(tokens_iter.peek().unwrap(), Token::CloseBrace) {
                        let statement = match parse_statement(tokens_iter) {
                            Ok(val) => val,
                            Err(msg) => return Err(msg),
                        };
                        body.push(statement);
                    }
                    tokens_iter.next();
                    let function_statement = statements::Function::new(
                        statement_type,
                        String::from(function_name),
                        body,
                    );
                    return Ok(function_statement);
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

pub fn parse_program_tokens(tokens: Vec<Token>) -> Result<Program, String> {
    let mut token_iterator = tokens.into_iter().peekable();
    let main_function: Statement = match parse_statement(&mut token_iterator) {
        Ok(main_statement) => match &main_statement {
            Statement::Function(val) => {
                if val.name != "main" {
                    return Err("Missing main function".to_string());
                }
                main_statement
            }
            _ => return Err("Missing main function".to_string()),
        },
        Err(msg) => return Err(msg),
    };
    let main_program: Program = Program::new(main_function);
    return Ok(main_program);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expression_tokens() {
        let tokens: Vec<Token> = vec![Token::Integer("2"), Token::Semicolon];
        let mut token_iterator = tokens.into_iter().peekable();
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
        let mut token_iterator = tokens.into_iter().peekable();
        let return_statement: Statement = match parse_statement(&mut token_iterator) {
            Ok(val) => val,
            Err(msg) => panic!("{}", msg),
        };
        let correct_expression = test_utils::create_test_return_statement();
        let correct_format = format!("{}", correct_expression);
        let test_format = format!("{}", return_statement);
        assert_eq!(correct_format, test_format);
    }

    #[test]
    fn test_parse_function_statement_tokens() {
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
        let mut token_iterator = tokens.into_iter().peekable();
        let function_node: Statement = match parse_statement(&mut token_iterator) {
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
        let program: Program = match parse_program_tokens(tokens) {
            Ok(program) => program,
            Err(msg) => panic!("{}", msg),
        };
        assert_eq!(
            format!("{}", test_utils::create_test_program()),
            format!("{}", program)
        );
    }
}
