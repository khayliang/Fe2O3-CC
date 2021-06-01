use crate::syntax::*;

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
