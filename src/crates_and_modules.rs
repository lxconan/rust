mod front_of_the_house {
  pub mod hosting {
    pub fn _return_true_from_hosting() -> bool { true }
  }
}

fn _return_true_from_super() -> bool { true }

#[cfg(test)]
mod test {
  mod child_module {
    pub fn return_true_from_child() -> bool { true }

    pub struct User {
      pub name: String,
      pub year_of_birth: u16
    }

    pub enum Color {
      Rgb {red: u8, green: u8, blue: u8}
    }
  }

  #[test]
  fn should_access_public_functions_using_absolute_paths() {
    let return_value = crate::crates_and_modules::front_of_the_house::hosting::_return_true_from_hosting();
    assert_eq!(true, return_value);
  }

  #[test]
  fn should_access_public_functions_using_relative_paths() {
    let return_value = child_module::return_true_from_child();
    assert_eq!(true, return_value);
  }

  #[test]
  fn should_access_parent_module_using_super() {
    let return_value = super::_return_true_from_super();
    assert_eq!(true, return_value);
  }

  #[test]
  fn should_specify_visibility_for_each_field_for_struct() {
    // try removing one of the pub modifier.
    let user = child_module::User { name: String::from("John"), year_of_birth: 1909 };
    assert_eq!("John", user.name.as_str());
    assert_eq!(1909, user.year_of_birth);
  }

  #[test]
  fn should_auto_public_for_variations_of_enum() {
    let green = child_module::Color::Rgb { red: 0, green: 255, blue: 0 };
    let child_module::Color::Rgb { red, green, blue } = green;
    assert_eq!(0, red);
    assert_eq!(255, green);
    assert_eq!(0, blue);
  }

  use std::io::Result as IoResult;

  fn bad() -> Option<&'static str> { Option::Some("Bad") }

  #[test]
  fn should_rename_with_as_keyword() {
    let result = IoResult::Ok("Good");
    assert_eq!(Some("Good"), result.ok().or_else(bad));
  }

  use rand::Rng;

  #[test]
  fn should_use_external_package() {
    let number = rand::thread_rng().gen_range(1, 101);
    assert!(number >= 1 && number <= 100);
  }
}