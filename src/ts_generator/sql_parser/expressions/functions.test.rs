#[cfg(test)]
mod tests {
    extern crate rand;
    use crate::ts_generator::sql_parser::expressions::functions::{
        is_date_function, is_numeric_function, is_string_function, DATE_FUNCTIONS, NUMERIC_FUNCTIONS, STRING_FUNCTIONS,
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
}
