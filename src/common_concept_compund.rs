#[cfg(test)]
mod test {
  #[test]
  fn should_declare_and_get_value_of_tuple() {
    let group:(i32, &str) = (32, "good");

    assert_eq!(32, group.0);
    assert_eq!("good", group.1);
  }

  #[test]
  fn should_get_value_from_tuple_using_destructing() {
    let group:(i32, &str) = (32, "good");
    let (number, message) = group;

    assert_eq!(32, number);
    assert_eq!("good", message);
  }

  #[test]
  fn should_declare_array_using_square_brakets() {
    let array = [1, 2, 3, 4, 5];
    assert_eq!(3, array[2]);
  }

  #[test]
  fn should_declare_array_by_specifying_type_and_length () {
    let array:[i32; 5] = [1, 2, 3, 4, 5];

    assert_eq!(3, array[2]);
    assert_eq!(20, std::mem::size_of::<[i32; 5]>());
  }

  #[test]
  fn should_declear_array_with_same_elements() {
    let array = [3; 5];

    assert_eq!(3, array[0]);
    assert_eq!(3, array[1]);
    assert_eq!(3, array[2]);
    assert_eq!(3, array[3]);
    assert_eq!(3, array[4]);
  }

  #[test]
  fn should_be_able_to_get_length_of_the_array() {
    let array = [3; 5];

    assert_eq!(5, array.len());
  }
}