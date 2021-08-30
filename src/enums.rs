#[cfg(test)]
mod test{
  enum IpAddress {
    V4
  }

  #[test]
  fn should_move_enum_value() {
    let v4 = IpAddress::V4;
    let _moved = v4;

    // try uncomment the following line.
    // let another = v4;
  }

  enum IpAddressWithData {
    V4(u8, u8, u8, u8),
    V6(String)
  }

  impl IpAddressWithData {
    fn to_string (&self) -> String {
      match self {
        IpAddressWithData::V4(p1, p2, p3, p4) => format!("{}.{}.{}.{}", p1, p2, p3, p4),
        IpAddressWithData::V6(value) => String::from(value),
      }
    }

    fn is_loop_back (&self) -> bool {
      match self {
        IpAddressWithData::V4(127, 0, 0, 1) => true,
        IpAddressWithData::V6(value) => value.eq("::1"),
        _ => false
      }
    }
  }

  #[test]
  fn should_define_and_match_each_variants_in_enum() {
    let v4 = IpAddressWithData::V4(127, 0, 0, 1);
    let v6 = IpAddressWithData::V6(String::from("::1"));

    assert_eq!("127.0.0.1", v4.to_string());
    assert_eq!("::1", v6.to_string());
  }

  #[test]
  fn shoud_match_exact_value_in_enum() {
    let loopback_v4 = IpAddressWithData::V4(127, 0, 0, 1);
    let loopback_v6 = IpAddressWithData::V6(String::from("::1"));
    let normal_v4 = IpAddressWithData::V4(192, 168, 1, 1);

    assert_eq!(true, loopback_v4.is_loop_back());
    assert_eq!(true, loopback_v6.is_loop_back());
    assert_eq!(false, normal_v4.is_loop_back());
  }

  #[test]
  fn should_use_if_let_to_avoid_exhaust_matching() {
    let loopback_v4 = IpAddressWithData::V4(127, 0, 0, 1);
    let mut is_loop_back = false;
    if let IpAddressWithData::V4(127, 0, 0, 1) = loopback_v4 {
      is_loop_back = true;
    }

    assert_eq!(true, is_loop_back);
  }

  #[test]
  fn should_be_aware_that_it_is_matching_rather_than_equality_comparison() {
    let loopback_v4 = IpAddressWithData::V4(222, 222, 222, 222);
    let mut is_match = false;
    if let IpAddressWithData::V4(_, _, _, _) = loopback_v4 {
      is_match = true;
    }

    assert_eq!(true, is_match);
  }

  #[test]
  fn should_apply_match_and_capture() {
    let loopback_v4 = IpAddressWithData::V4(222, 222, 222, 222);
    let mut part1:u8 = 0;
    if let IpAddressWithData::V4(p1, _, _, _) = loopback_v4 {
      part1 = p1;
    }

    assert_eq!(222, part1);
  }
}