use crate::syntax::*;
use crate::tokenizer::Token;

fn string_to_number(s: &String) -> i32 {
    let mut error_str = "Invalid_syntax: ".to_string();
    error_str.push_str(s);
    s.parse::<i32>().expect(&error_str)
}

fn parse_expression(tokens: &[Token]) -> Result<Box<dyn Expression>, String> {
    let mut tokens_iter = tokens.iter().enumerate().peekable();
    while let Some((_, token)) = tokens_iter.next(){
        match token {
            Token::Integer(val) => {
                let int_variable = Type::Integer(string_to_number(&val));
                let (_, next_token) = *tokens_iter.peek().unwrap();
                if *next_token == Token::Semicolon {
                    return Ok(Box::new(expressions::Constant::new(int_variable)))
                }
            }
            _ => {
                let mut msg = String::from("Unidentified expression");
                msg.push_str(format!("{:?}", token).as_str());
                return Err(msg)
            }
        }
    }
    return Err(String::from("Something went wrong"))
}
/*
pub fn parse_statement(tokens: &[Token]) -> Result<Box<dyn Statement>, String> {

}
 */
/* 
fn parse(tokens: Vec<Token>) -> Program {

}
*/

#[cfg(test)]
mod tests {
    use super::*;
    fn create_test_integer() -> Type {
        Type::Integer(2)
    }

    fn create_test_constant_expression() -> expressions::Constant {
        expressions::Constant::new(create_test_integer())
    }

    fn create_test_return_statement() -> statements::Return {
        statements::Return::new(Box::new(create_test_constant_expression()))
    }

    fn create_test_function() -> Function {
        let identifier = String::from("main");
        let body: Vec<Box<dyn Statement>> = vec![Box::new(create_test_return_statement())];
        let return_type = Type::Integer(0);
        Function::new(return_type, identifier, body)
    }

    fn create_test_program() -> Program {
        Program::new(create_test_function())
    }

    #[test]
    fn test_parse_expression_tokens(){
        let tokens: Vec<Token> = vec![
            Token::Integer("2".to_string()),
            Token::Semicolon,
        ];
        let expression: Box<dyn Expression> = match parse_expression(&tokens){
            Ok(val) => val,
            Err(msg) => panic!("{}", msg),
        };
        let correct_expression = create_test_constant_expression();
        let correct_format = format!("{}", correct_expression);
        let test_format = format!("{}", expression);
        assert_eq!(correct_format, test_format);
    }

    #[test]
    fn test_parse_statement_tokens(){

    }

    #[test]
    fn test_parse_function_tokens_into_function() {

    }

    #[test]
    fn test_parse_all_tokens_into_program_with_main() {
        let tokens: Vec<Token> = vec![
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
        /*
        let program: Program = super::parse_program(tokens);
        let main_function: Function = program.root;
        assert_eq!(main_function.name, String::from("main"));
        assert!(std::mem::discriminant(&main_function.return_type) == std::mem::discriminant(&Type::Integer(0)));
        */
    }
}

