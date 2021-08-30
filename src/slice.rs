#[cfg(test)]
mod test {
  #[test]
  fn should_create_string_slice() {
    let message = String::from("Hello");
    let slice:&str = &message[1..3];
    
    assert_eq!("el", slice);
  }

  #[test]
  fn should_reference_whole_string_as_str_using_slice() {
    let message = String::from("Hello");
    let slice = &message[..];
    
    assert_eq!("Hello", slice);
  }

  #[test]
  fn should_create_slice_according_to_element_index() {
    let array = [1, 2, 3, 4, 5];
    let two_and_three:&[i32] = &array[1..3];

    assert_eq!(2, two_and_three[0]);
    assert_eq!(3, two_and_three[1]);
  }

  #[test]
  fn should_reflect_slice_modify() {
    let mut array = [1, 2, 3, 4, 5];
    let two_and_three:&mut[i32] = &mut array[1..3];

    two_and_three[1] = 9;

    assert_eq!(1, array[0]);
    assert_eq!(2, array[1]);
    assert_eq!(9, array[2]);
    assert_eq!(4, array[3]);
    assert_eq!(5, array[4]);
  }
}