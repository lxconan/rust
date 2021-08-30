#[cfg(test)]
mod test {
  use std::mem::size_of;

  #[test]
  fn should_represent_a_unicode_32_character() {
    let english_charater = 'A';
    let chinese_character = '占';
    let surrogate_pair:char = '😹';

    assert_eq!('\u{41}', english_charater);
    assert_eq!('\u{5360}', chinese_character);
    assert_eq!('\u{1F639}', surrogate_pair);
    assert_eq!(4, size_of::<char>());
  }
}