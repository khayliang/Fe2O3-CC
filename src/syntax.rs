use std::fmt;

use textwrap::indent;

pub trait Node: fmt::Display {
    fn type_of(&self) -> &'static str;
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Type {
    Integer(i32),
}
impl Type {
    fn value_to_string(&self) -> String {
        match self {
            Self::Integer(val) => return format!("{}", val),
        }
    }
}
impl Node for Type {
    fn type_of(&self) -> &'static str {
        match self {
            Self::Integer(_) => "Integer",
        }
    }
}
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_string = self.type_of();
        let formatted_string = format!("{}<{}>", type_string, self.value_to_string());
        write!(f, "{}", formatted_string)
    }
}

pub trait Expression: Node {
    fn evaluate(&self) -> Type;
}
pub mod expressions {
    use super::*;

    #[derive(Debug)]
    pub struct Constant {
        value: Type,
    }
    impl Constant {
        pub fn new(value: Type) -> Constant {
            Constant { value }
        }
    }
    impl Node for Constant {
        fn type_of(&self) -> &'static str {
            "Constant"
        }
    }
    impl Expression for Constant {
        fn evaluate(&self) -> Type {
            return self.value;
        }
    }
    impl fmt::Display for Constant {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let type_string = self.type_of();
            let formatted_string = format!("{} {}", type_string, self.value);
            write!(f, "{}", formatted_string)
        }
    }
}
pub trait Statement: Node {}
pub mod statements {
    use super::*;
    pub struct Return {
        pub expression: Box<dyn Expression>,
    }
    impl Return {
        pub fn new(expression: Box<dyn Expression>) -> Return {
            Return { expression }
        }
    }
    impl Node for Return {
        fn type_of(&self) -> &'static str {
            "Return"
        }
    }
    impl Statement for Return {}
    impl fmt::Display for Return {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let formatted_string = format!("{} {}", self.type_of(), self.expression);
            write!(f, "{}", formatted_string)
        }
    }
}

pub struct Function {
    pub return_type: Type,
    pub name: String,
    pub body: Vec<Box<dyn Statement>>,
}
impl Function {
    pub fn new(return_type: Type, name: String, body: Vec<Box<dyn Statement>>) -> Function {
        Function {
            return_type,
            name: name,
            body,
        }
    }
}
impl Node for Function {
    fn type_of(&self) -> &'static str {
        "Function"
    }
}
impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut formatted_statements_body = String::new();
        for statement in self.body.iter() {
            let formatted_statement = format!("{}\n", statement);
            formatted_statements_body.push_str(&formatted_statement);
        }
        formatted_statements_body = indent(&formatted_statements_body, "        ");
        let formatted_function = format!(
            "Function {} {}:\n    body:\n{}",
            self.return_type.type_of(),
            self.name,
            formatted_statements_body
        );
        write!(f, "{}", formatted_function)
    }
}
pub struct Program {
    pub root: Function,
}
impl Program {
    pub fn new(root: Function) -> Program {
        Program { root }
    }
}
impl Node for Program {
    fn type_of(&self) -> &'static str {
        "Program"
    }
}
impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PROGRAM_START:\n{}", self.root)
    }
}

pub mod test_utils {
    use super::*;

    pub fn create_test_integer() -> Type {
        Type::Integer(2)
    }

    pub fn create_test_constant_expression() -> expressions::Constant {
        expressions::Constant::new(create_test_integer())
    }

    pub fn create_test_return_statement() -> statements::Return {
        statements::Return::new(Box::new(create_test_constant_expression()))
    }

    pub fn create_test_function() -> Function {
        let identifier = String::from("main");
        let body: Vec<Box<dyn Statement>> = vec![Box::new(create_test_return_statement())];
        let return_type = Type::Integer(0);
        Function::new(return_type, identifier, body)
    }

    pub fn create_test_program() -> Program {
        Program::new(create_test_function())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::test_utils::*;
    use indoc::indoc;

    #[test]
    fn test_integer_variable() {
        let integer = create_test_integer();
        let integer_formatted: String = format!("{}", integer);
        assert_eq!("Integer", integer.type_of());
        assert_eq!("Integer<2>", &integer_formatted);
    }

    #[test]
    fn test_constant_expression() {
        let integer = create_test_integer();
        let constant = create_test_constant_expression();
        let constant_formatted: String = format!("{}", constant);
        assert_eq!(integer, constant.evaluate());
        assert_eq!("Constant", constant.type_of());
        assert_eq!("Constant Integer<2>", constant_formatted);
    }

    #[test]
    fn test_return_statement() {
        let return_statement = create_test_return_statement();
        let return_formatted: String = format!("{}", return_statement);
        assert_eq!("Return", return_statement.type_of());
        assert_eq!("Return Constant Integer<2>", return_formatted);
    }

    #[test]
    fn test_function() {
        let function = create_test_function();
        let function_formatted: String = format!("{}", function);
        let expected_format = indoc! {"
            Function Integer main:
                body:
                    Return Constant Integer<2>
        "};
        assert_eq!(expected_format, function_formatted);
    }

    #[test]
    fn test_program() {
        let main_program = create_test_program();
        let expected_format = indoc! {"
            PROGRAM_START:
            Function Integer main:
                body:
                    Return Constant Integer<2>
        "};
        let program_formatted: String = format!("{}", main_program);
        assert_eq!(expected_format, program_formatted);
    }
}
