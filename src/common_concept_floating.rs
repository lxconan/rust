#[cfg(test)]
mod test {
  #[test]
  fn should_perform_float_calculations() {
    let floating = 5.0;
    assert_eq!(2.5, floating / 2.0);
  }

  #[test]
  fn should_cast_before_mix_integer_and_floating_calculations() {
    let integer = 2;
    let floating = 3.0;

    assert_eq!(6.0, (integer as f64) * floating);
  }
}