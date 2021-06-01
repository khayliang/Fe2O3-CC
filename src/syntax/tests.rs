
pub mod test_utils {
  use crate::syntax::*;

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
  use crate::syntax::*;
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
