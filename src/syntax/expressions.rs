use crate::syntax::*;

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