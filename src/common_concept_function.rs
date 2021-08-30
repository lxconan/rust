#[cfg(test)]
mod test {
  fn function_with_parameter(number:i32) -> i32 {
    return number;
  }

  #[test]
  fn should_pass_parameter_to_function() {
    assert_eq!(3, function_with_parameter(3));
  }

  fn function_with_mutable_parameter(mut number: i32) -> i32 {
    println!("input: {}", number);
    number = 64;
    return number;
  }

  #[test]
  fn should_pass_by_value() {
    let argument = 32;
    let return_value = function_with_mutable_parameter(argument);
    assert_eq!(32, argument);
    assert_eq!(64, return_value);
  }

  fn return_by_expression() -> i32 { 5 }

  fn pass_array_by_value(mut array:[i32;5]) {
    for i in 0..5 {
      array[i] = 3;
    }
  }

  #[test]
  fn should_pass_array_by_value() {
    let array:[i32; 5] =[0, 1, 2, 3, 4];
    pass_array_by_value(array);
    assert_eq!(0, array[0]);
    assert_eq!(1, array[1]);
    assert_eq!(2, array[2]);
    assert_eq!(3, array[3]);
    assert_eq!(4, array[4]);
  }

  fn pass_array_by_reference(array:&mut [i32;5]) {
    for i in 0..5 {
      array[i] = 3;
    }
  }

  #[test]
  fn should_pass_array_by_reference() {
    let mut array:[i32; 5] =[0, 1, 2, 3, 4];
    pass_array_by_reference(&mut array);
    for i in 0..5 {
      assert_eq!(3, array[i]);
    }
  }

  #[test]
  fn should_use_expression_to_return_value() {
    assert_eq!(5, return_by_expression())
  }

  #[test]
  fn should_evaluate_expression() {
    let x = 5;
    let y = { let temp = x + 3; temp * 5 };
    assert_eq!(40, y);
  }
}