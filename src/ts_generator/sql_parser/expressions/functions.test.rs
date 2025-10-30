#[cfg(test)]
mod tests {
  extern crate rand;
  use crate::ts_generator::sql_parser::expressions::functions::{
    is_date_function, is_numeric_function, is_string_function, is_type_polymorphic_function, DATE_FUNCTIONS,
    NUMERIC_FUNCTIONS, STRING_FUNCTIONS, TYPE_POLYMORPHIC_FUNCTIONS,
  };
  use rand::seq::SliceRandom;

  #[test]
  fn should_return_numeric_method_truthy() {
    let funcs = NUMERIC_FUNCTIONS.to_vec();
    let random_func = funcs.choose(&mut rand::thread_rng());
    let result = is_numeric_function(random_func.unwrap());
    assert!(result)
  }

  #[test]
  fn should_return_numeric_method_falsy() {
    let result = is_numeric_function("abcd");
    assert!(!result)
  }

  #[test]
  fn should_return_string_method_truthy() {
    let funcs = STRING_FUNCTIONS.to_vec();
    let random_func = funcs.choose(&mut rand::thread_rng());
    let result = is_string_function(random_func.unwrap());
    assert!(result)
  }

  #[test]
  fn should_return_string_method_falsy() {
    let result = is_string_function("abcd");
    assert!(!result)
  }

  #[test]
  fn should_return_date_method_truthy() {
    let funcs = DATE_FUNCTIONS.to_vec();
    let random_func = funcs.choose(&mut rand::thread_rng());
    let result = is_date_function(random_func.unwrap());
    assert!(result)
  }

  #[test]
  fn should_return_date_method_falsy() {
    let result = is_date_function("abcd");
    assert!(!result)
  }

  #[test]
  fn should_return_type_polymorphic_function_truthy() {
    let funcs = TYPE_POLYMORPHIC_FUNCTIONS.to_vec();
    let random_func = funcs.choose(&mut rand::thread_rng());
    let result = is_type_polymorphic_function(random_func.unwrap());
    assert!(result)
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
