#[cfg(test)]
mod test {
  fn get_calculated_value_if_greater_than_7(value: i32) -> i32 {
    if value > 7 {
      return value * 2;
    }

    0
  }

  #[test]
  fn should_apply_basic_if_control_flow() {
    let greater_than_7 = get_calculated_value_if_greater_than_7(8);
    let other = get_calculated_value_if_greater_than_7(7);

    assert_eq!(16, greater_than_7);
    assert_eq!(0, other);
  }

  fn get_integer_sign_flag(value: i32) -> i32 {
    if value == 0 {
      return 0;
    } else if value > 0 {
      return 1;
    } else {
      return -1;
    }
  }

  #[test]
  fn should_apply_complex_condition_using_else_if() {
    let positive = get_integer_sign_flag(100);
    let negative = get_integer_sign_flag(-100);
    let zero = get_integer_sign_flag(0);

    assert_eq!(1, positive);
    assert_eq!(-1, negative);
    assert_eq!(0, zero);
  }

  #[test]
  fn should_apply_condition_in_an_expression() {
    let value = 3;
    let sign_flag = if value > 0 { 1 } else if value < 0 { - 1 } else { 0 };
    assert_eq!(1, sign_flag);
  } 

  #[test]
  fn should_repeat_with_loop() {
    let values = [1, 2, 3];
    let mut sum = 0;
    let mut index = 0;
    loop {
      if index >= values.len() { break; }
      sum += values[index];
      index += 1;
    }

    assert_eq!(6, sum);
  }

  #[test]
  fn should_return_values_from_loop() {
    let values = [1, 2, 3];
    let mut index = 0;
    let mut sum = 0;

    let double_sum = loop {
      if index >= values.len() { break sum * 2; }
      sum += values[index];
      index += 1;
    };

    assert_eq!(12, double_sum);
  }

  #[test]
  fn should_repeat_with_for() {
    let values = [1, 2, 3];
    let mut sum = 0;
    for i in 0..values.len() {
      sum += values[i];
    }

    assert_eq!(6, sum);
  }

  #[test]
  fn should_repeat_with_for_iter() {
    let values = [1, 2, 3];
    let mut sum = 0;
    for value in values.iter() {
      sum += value;
    }

    assert_eq!(6, sum);
  }

  #[test]
  fn should_repeat_with_while() {
    let values = [1, 2, 3];
    let mut sum = 0;
    let mut index = 0;
    while index < values.len() {
      sum += values[index];
      index += 1;
    }

    assert_eq!(6, sum);
  }
}