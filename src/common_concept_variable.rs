#[cfg(test)]
mod test {
  #[test]
  fn should_define_variable_without_annotating_type_and_assign_value() {
    let number = 5;
    assert_eq!(5, number);
  }

  #[test]
  fn should_define_variable_with_specified_type_and_assign_value() {
    let number: i32 = 5;
    assert_eq!(5, number);
  }

  #[test]
  fn should_specify_type_explictly() {
    let number:i32 = "33".parse().expect("Not a number");
    let equivalent:i32 = "33".parse().expect("Not a number");
    assert_eq!(equivalent, number
    );
  }

  #[test]
  fn should_modify_variable_by_declearing_as_mutable() {
    let mut number = 4;
    assert_eq!(4, number);
    number = 5;
    assert_eq!(5, number);
  }

  #[test]
  fn should_define_constants() {
    const CONST_NUMBER: u32 = 5;
    assert_eq!(5, CONST_NUMBER);
  }

  #[test]
  fn should_be_able_to_shadow_variables() {
    // we use underscore to explictly tell compiler that this variable will be shadowed.
    let _number = 5;
    let _number = 6;
    assert_eq!(6, _number);
  }

  #[test]
  fn should_be_able_to_shadow_variables_by_leverage_its_old_value() {
    let _number = 5;
    let _number = _number * 3 + 1;
    let _number = _number * 2;

    assert_eq!(_number, 32);
  }

  #[test]
  fn should_be_able_to_change_shadow_variable_type() {
    let _number:i32 = 5;
    let _number:&str = "Hello";

    assert_eq!("Hello", _number);
  }
}
