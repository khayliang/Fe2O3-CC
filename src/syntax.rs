pub trait Node {}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Type {
    Integer(i32),
}

pub trait Expression {
    fn evaluate(&self) -> Type;
}
impl Node for dyn Expression {}
pub mod expressions {
    use super::*;
    pub struct Constant {
        value: Type,
    }
    impl Constant {
        pub fn new(value: Type) -> Constant {
            Constant { value }
        }
    }
    impl Expression for Constant {
        fn evaluate(&self) -> Type {
            return self.value;
        }
    }
}
pub trait Statement {}
impl Node for dyn Statement {}
pub mod statements {
    use super::*;
    pub struct Return {
        expression: Box<Expression>,
    }
    impl Return {
        pub fn new(expression: Box<Expression>) -> Return {
            Return { expression }
        }
    }
    impl Statement for Return {}
}

pub struct Function {
    return_type: Type,
    name: String,
    body: Vec<Box<Statement>>,
}
impl Node for Function {}
impl Function {
    pub fn new(return_type: Type, name: String, body: Vec<Box<Statement>>) -> Function {
        Function {
            return_type,
            name: name,
            body,
        }
    }
}

pub struct Program {
    root: Function,
}
impl Node for Program {}
impl Program {
    pub fn new(root: Function) -> Program {
        Program { root }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_variable() {
        let integer = Type::Integer(2);
        let integer_enum = Type::Integer;
    }

    #[test]
    fn test_constant_expression() {
        let integer = Type::Integer(2);
        let constant = expressions::Constant::new(integer);
        assert_eq!(integer, constant.evaluate());
    }

    #[test]
    fn test_program() {
        let integer = Type::Integer(2);
        let constant = expressions::Constant::new(integer);
        let return_statement = statements::Return::new(Box::new(constant));

        let identifier = String::from("main");
        let body: Vec<Box<Statement>> = vec![Box::new(return_statement)];
        let return_type = Type::Integer(0);
        let function = Function::new(return_type, identifier, body);
        let main_program = Program::new(function);
    }
}
