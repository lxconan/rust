#[cfg(test)]
mod test {
  struct User {
    username: String,
    year_of_birth: u16
  }

  #[test]
  fn should_define_struct() {
    let user = User {
      username: String::from("John"),
      year_of_birth: 1999
    };

    assert_eq!("John", user.username);
    assert_eq!(1999, user.year_of_birth);
  }

  #[test]
  fn should_define_struct_by_short_hand() {
    let username = String::from("John");
    let year_of_birth = 1999;

    let user = User { username, year_of_birth };

    assert_eq!("John", user.username);
    assert_eq!(1999, user.year_of_birth);
  }

  #[test]
  fn should_define_tuple_structs() {
    struct RgbColor(f32, f32, f32);
    struct HsbColor(f32, f32, f32);

    assert_ne!(std::any::type_name::<RgbColor>(), std::any::type_name::<HsbColor>());
  }

  #[test]
  fn should_mark_instance_as_mutable_to_modify_fields() {
    let mut user = User {
      username: String::from("John"),
      year_of_birth: 1999
    };

    user.username = String::from("Marry");
    user.year_of_birth = 2002;

    assert_eq!("Marry", user.username);
    assert_eq!(2002, user.year_of_birth);
  }

  struct Rectangle {
    width: u32,
    height: u32
  }

  impl Rectangle {
    fn area(&self) -> u32 {
      self.width * self.height
    }

    fn can_hold(&self, other:&Rectangle) -> bool {
      self.width >= other.width && self.height >= other.height
    }

    fn square(size: u32) -> Rectangle {
      Rectangle { width: size, height: size }
    }
  }

  #[test]
  fn should_define_method_in_struct() {
    assert_eq!(32, Rectangle{width:4, height:8}.area())
  }

  #[test]
  fn should_define_method_with_parameters() {
    assert_eq!(true, Rectangle{width: 10, height: 20}.can_hold(&Rectangle{width: 5, height: 2}));
    assert_eq!(false, Rectangle{width: 10, height: 20}.can_hold(&Rectangle{width: 5, height: 21}));
  }

  #[test]
  fn should_call_associate_functions() {
    let square = Rectangle::square(10);
    assert_eq!(10, square.width);
    assert_eq!(10, square.height);
  }
}