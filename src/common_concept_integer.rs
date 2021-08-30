#[cfg(test)]
mod test {
  #[test]
  fn should_overflow_at_release_mode_and_throw_at_debug_mode() {
    let byte:i8 = "127".parse().expect("Not a byte");
    let one:i8 = 1;
    // the test will fail in debug mode. You can use `cargo test --release`
    // to test it in release mode.
    assert_eq!(-128, byte + one);
  }

  #[test]
  fn should_be_able_to_use_suffix_to_specify_integer_type() {
    let byte = b'A';
    let four_byte_integer = 32i32;
    let four_byte_unsigned_integer = 32u32;

    assert_eq!(65, byte);
    assert_eq!(32, four_byte_integer);
    assert_eq!(32, four_byte_unsigned_integer);
  }

  #[test]
  fn should_declare_floating_point_number_() {
    let double_equivalent = 2.0;
    let float_equivalent:f32 = 2.0;

    assert_eq!(2.0, double_equivalent);
    assert_eq!(2.0f32, float_equivalent);
  }

  #[test]
  fn should_perform_integer_calculations() {
    let interger = 5;
    assert_eq!(2, interger / 2);
    assert_eq!(10, interger * 2);
    assert_eq!(2, interger % 3);
  }

  #[test]
  fn should_perform_bitwise_operations() {
    let bitwise_not:u32 = !0xfffffffc;
    let bitwise_and:u32 = 0xffff0000 & 0x0000ffff;
    let bitwise_or:u32 = 0xffff0000 | 0x0000ffff;
    let left_shift:u32 = 0x00000001 << 1;
    let left_shift_overflow:u32 = 0x80000000 << 1;
    let right_shift:u32 = 0x20000000 >> 1;
    let right_shift_overflow:u32 = 0x00000001 >> 1;

    assert_eq!(3, bitwise_not);
    assert_eq!(0, bitwise_and);
    assert_eq!(0xffffffff, bitwise_or);
    assert_eq!(0x00000002, left_shift);
    assert_eq!(0x00000000, left_shift_overflow);
    assert_eq!(0x10000000, right_shift);
    assert_eq!(0x00000000, right_shift_overflow);
  }
}