#[cfg(test)]
mod tests {
  use crate::ts_generator::sql_parser::expressions::functions::{
    is_date_function, is_numeric_function, is_string_function, is_type_polymorphic_function,
  };

  #[test]
  fn should_return_numeric_method_truthy() {
    // Test with a known numeric function
    let result = is_numeric_function("ABS");
    assert!(result);
    let result = is_numeric_function("COUNT");
    assert!(result);
  }

  #[test]
  fn should_return_numeric_method_falsy() {
    let result = is_numeric_function("abcd");
    assert!(!result)
  }

  #[test]
  fn should_return_string_method_truthy() {
    // Test with a known string function
    let result = is_string_function("CONCAT");
    assert!(result);
    let result = is_string_function("UPPER");
    assert!(result);
  }

  #[test]
  fn should_return_string_method_falsy() {
    let result = is_string_function("abcd");
    assert!(!result)
  }

  #[test]
  fn should_return_date_method_truthy() {
    // Test with a known date function
    let result = is_date_function("NOW");
    assert!(result);
    let result = is_date_function("DATE");
    assert!(result);
  }

  #[test]
  fn should_return_date_method_falsy() {
    let result = is_date_function("abcd");
    assert!(!result)
  }

  #[test]
  fn should_return_type_polymorphic_function_truthy() {
    // Test with a known type-polymorphic function
    let result = is_type_polymorphic_function("IFNULL");
    assert!(result);
    let result = is_type_polymorphic_function("COALESCE");
    assert!(result);
  }

  #[test]
  fn should_return_type_polymorphic_function_falsy() {
    let result = is_type_polymorphic_function("abcd");
    assert!(!result)
  }

  #[test]
  fn should_recognize_ifnull_as_type_polymorphic() {
    assert!(is_type_polymorphic_function("IFNULL"));
    assert!(is_type_polymorphic_function("ifnull"));
    assert!(is_type_polymorphic_function("IfNull"));
  }

  #[test]
  fn should_recognize_coalesce_as_type_polymorphic() {
    assert!(is_type_polymorphic_function("COALESCE"));
    assert!(is_type_polymorphic_function("coalesce"));
  }

  #[test]
  fn should_recognize_nullif_as_type_polymorphic() {
    assert!(is_type_polymorphic_function("NULLIF"));
    assert!(is_type_polymorphic_function("nullif"));
  }
}
