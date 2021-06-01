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

pub mod expressions;
pub mod statements;
pub mod tests;