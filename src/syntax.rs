use std::fmt;

use indoc::formatdoc;
use textwrap::indent;

pub trait Node: fmt::Display {
    fn type_of(&self) -> &'static str;
    fn to_asm(&self) -> String;
}

// TODO: Create a variable struct that has type Type Enum
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
    fn to_asm(&self) -> String {
        match self {
            Self::Integer(val) => return format!("${}", val),
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

pub enum Expression {
    Constant(expressions::Constant),
}
pub trait ExpressionTrait: Node {
    fn evaluate(&self) -> Type;
}
impl ExpressionTrait for Expression {
    fn evaluate(&self) -> Type {
        match self {
            Self::Constant(val) => val.evaluate(),
        }
    }
}
impl Node for Expression {
    fn type_of(&self) -> &'static str {
        match self {
            Self::Constant(val) => val.type_of(),
        }
    }
    fn to_asm(&self) -> String {
        match self {
            Self::Constant(val) => val.to_asm(),
        }
    }
}
impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Constant(val) => write!(f, "{}", val),
        }
    }
}

pub mod expressions {
    use super::*;

    #[derive(Debug)]
    pub struct Constant {
        value: Type,
    }
    impl Constant {
        pub fn new(value: Type) -> Expression {
            Expression::Constant(Constant { value })
        }
    }
    impl Node for Constant {
        fn type_of(&self) -> &'static str {
            "Constant"
        }
        fn to_asm(&self) -> String {
            return self.value.to_asm();
        }
    }
    impl ExpressionTrait for Constant {
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

pub enum Statement {
    Function(statements::Function),
    Return(statements::Return),
}
trait StatementTrait: Node {}
impl StatementTrait for Statement {}
impl Node for Statement {
    fn type_of(&self) -> &'static str {
        match self {
            Self::Function(val) => val.type_of(),
            Self::Return(val) => val.type_of(),
        }
    }
    fn to_asm(&self) -> String {
        match self {
            Self::Function(val) => val.to_asm(),
            Self::Return(val) => val.to_asm(),
        }
    }
}
impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Function(val) => write!(f, "{}", val),
            Self::Return(val) => write!(f, "{}", val),
        }
    }
}

pub mod statements {
    use super::*;
    pub struct Return {
        pub expression: Expression,
    }
    impl Return {
        pub fn new(expression: Expression) -> Statement {
            Statement::Return(Return { expression })
        }
    }
    impl StatementTrait for Return {}
    impl Node for Return {
        fn type_of(&self) -> &'static str {
            "Return"
        }
        fn to_asm(&self) -> String {
            let return_expression_asm = self.expression.to_asm();
            let return_asm = formatdoc! {"
                movl {}, %eax
                ret
                ",
                return_expression_asm
            };
            return return_asm;
        }
    }

    impl fmt::Display for Return {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let formatted_string = format!("Return {}", self.expression);
            write!(f, "{}", formatted_string)
        }
    }

    pub struct Function {
        pub return_type: Type,
        pub name: String,
        pub body: Vec<Statement>,
    }
    impl Function {
        pub fn new(return_type: Type, name: String, body: Vec<Statement>) -> Statement {
            Statement::Function(Function {
                return_type,
                name: name,
                body,
            })
        }
    }
    impl StatementTrait for Function {}
    impl Node for Function {
        fn type_of(&self) -> &'static str {
            "Function"
        }
        fn to_asm(&self) -> String {
            let asm_body: Vec<String> = self
                .body
                .iter()
                .map(|statement| statement.to_asm())
                .rev()
                .collect();
            let mut function_asm = formatdoc! {"
                .globl {name}
                {name}:
            ", name=self.name};
            asm_body.iter().for_each(|asm| function_asm.push_str(&asm));
            return function_asm;
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
}

pub struct Program {
    pub root: statements::Function,
}
impl Program {
    pub fn new(main_statement: Statement) -> Program {
        match main_statement {
            Statement::Function(func) => Program { root: func },
            _ => panic!("Missing main function!"),
        }
    }
}
impl Node for Program {
    fn type_of(&self) -> &'static str {
        "Program"
    }
    fn to_asm(&self) -> String {
        self.root.to_asm()
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

    pub fn create_test_constant_expression() -> Expression {
        expressions::Constant::new(create_test_integer())
    }

    pub fn create_test_return_statement() -> Statement {
        statements::Return::new(create_test_constant_expression())
    }

    pub fn create_test_function() -> Statement {
        let identifier = String::from("main");
        let body: Vec<Statement> = vec![create_test_return_statement()];
        let return_type = Type::Integer(0);
        statements::Function::new(return_type, identifier, body)
    }

    pub fn create_test_program() -> Program {
        Program::new(create_test_function())
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::*;
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_integer_variable() {
        let integer = create_test_integer();
        let integer_formatted: String = format!("{}", integer);
        assert_eq!("Integer", integer.type_of());
        assert_eq!("Integer<2>", &integer_formatted);
        assert_eq!("$2", integer.to_asm());
    }

    #[test]
    fn test_constant_expression() {
        let integer = create_test_integer();
        let constant = create_test_constant_expression();
        let constant_formatted: String = format!("{}", constant);
        assert_eq!(integer, constant.evaluate());
        assert_eq!("Constant", constant.type_of());
        assert_eq!("Constant Integer<2>", constant_formatted);
        assert_eq!("$2", constant.to_asm());
    }

    #[test]
    fn test_return_statement() {
        let return_statement = create_test_return_statement();
        let return_formatted: String = format!("{}", return_statement);
        assert_eq!("Return", return_statement.type_of());
        assert_eq!("Return Constant Integer<2>", return_formatted);
        let expected_asm = indoc! {"
            movl $2, %eax
            ret
        "};
        assert_eq!(expected_asm, return_statement.to_asm());
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
        let expected_asm = indoc! {"
            .globl main
            main:
            movl $2, %eax
            ret
        "};
        assert_eq!(expected_format, function_formatted);
        assert_eq!(expected_asm, function.to_asm());
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
        let expected_asm = indoc! {"
            .globl main
            main:
            movl $2, %eax
            ret
        "};
        assert_eq!(expected_asm, main_program.to_asm());
    }

    #[test]
    #[should_panic]
    fn create_new_program_fails() {
        let main_program = create_test_return_statement();
        Program::new(main_program);
    }
}
