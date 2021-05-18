pub mod expressions {
    pub struct Constant {
        value: u32,
    }
    impl Constant {
        pub fn new(value: u32) -> Constant {
            Constant { value }
        }
    }
}

pub mod statements {
    pub struct Return {
        value: super::expressions::Constant,
    }
    impl Return {
        pub fn new(value: super::expressions::Constant) -> Return {
            Return { value }
        }
    }
}

pub struct Function {
    name: String,
    body: statements::Return,
}
impl Function {
    pub fn new(name: &str, body: statements::Return) -> Function {
        Function {
            name: name.to_string(),
            body,
        }
    }
}

pub struct Program {
    value: Function,
}
impl Program {
    pub fn new(value: Function) -> Program {
        Program { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_node() {
        let expression = expressions::Constant::new(2);
        let statement = statements::Return::new(expression);
        let main_function = Function::new("main", statement);
        let main_program = Program::new(main_function);
    }
}
