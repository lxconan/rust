#[cfg(test)]
mod test {
  #[test]
  fn should_do_boolean_operations() {
    let true_value = true;
    assert_eq!(false, !true_value);
    assert_eq!(true, !!true_value);
    assert_eq!(true, true & true);
    assert_eq!(true, true && true);
    assert_eq!(false, true & false);
    assert_eq!(false, true && false);
    assert_eq!(true, true | false);
    assert_eq!(true, true || false);
    assert_eq!(false, false | false);
    assert_eq!(false, false || false);
  }
}