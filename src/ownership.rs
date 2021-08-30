#[cfg(test)]
mod test {
  // TODO: owner ship with life time events

  fn will_not_take_ownership(string:&String) -> usize {
    return string.len();
  }

  #[test]
  fn should_reference_variable_without_moving() {
    let original:String = String::from("Hello");
    let length = will_not_take_ownership(&original);

    assert_eq!("Hello", original.as_str());
    assert_eq!(5, length);
  }

  fn modify_mutable_reference(string:&mut String) {
    string.push_str(" World");
  }

  #[test]
  fn should_modify_mutable_reference() {
    let mut original:String = String::from("Hello");
    modify_mutable_reference(&mut original);

    assert_eq!("Hello World", original.as_str());
  }

  #[test]
  fn should_not_place_mutable_reference_to_the_same_object_twice_in_a_scope() {
    let mut original:String = String::from("Hello");

    {
      let borrowed_in_another_scope = &mut original;
      borrowed_in_another_scope.push_str(" World");
    }

    // If we move the following line to the top of the program, it will be a compiler error.
    let borrowed = &mut original;

    borrowed.push_str("!");

    assert_eq!("Hello World!", borrowed.as_str());
  }

  #[test]
  fn should_be_okay_to_place_multiple_immutable_references_in_scope() {
    // let mut original:String will also be fine (compiler warning)
    let original:String = String::from("Hello");

    let borrowed1 = &original;
    let borrowed2 = &original;

    assert_eq!("Hello", borrowed1.as_str());
    assert_eq!("Hello", borrowed2.as_str());
  }

  #[test]
  fn should_not_mix_immutable_and_mutable_reference_in_scope() {
    let mut original:String = String::from("Hello");
    
    original.push_str(" World"); // quickly out of scope
    let borrowed1 = &original;

    // Try uncomment the following line.
    // original.push_str(" World");

    assert_eq!("Hello World", borrowed1.as_str());
  }

  #[test]
  fn should_not_place_multiple_mutable_slice_references_in_scope() {
    let mut original = [1, 2, 3, 4, 5];

    let two_and_three:&mut[i32] = &mut original[1..3];
    // Try uncomment the following code.
    // let four_and_five:&mut[i32] = &mut original[3..5];

    assert_eq!(3, two_and_three[1]);
    // Try uncomment the following code.
    // assert_eq!(4, four_and_five[0]);
  }
}