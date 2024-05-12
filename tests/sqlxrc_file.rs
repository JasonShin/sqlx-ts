/// Test suites for converting any case to camelCase if generateTypes.convertToCamelCase is true
/// 
#[cfg(test)]
mod string_functions_tests {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::env;
    use std::fs;
    use std::io::Write;
    use std::process::Command;
    use tempfile::tempdir;
    

    use pretty_assertions::assert_eq;
    use test_utils::test_utils::TSString;
    use test_utils::{run_test, sandbox::TestConfig};


    #[rustfmt::skip]
run_test!(retain_original, TestConfig::new("postgres", false, Some(".sqlxrc.enabled.json".to_string())),
//// TS query ////
r#"
const someQuery = sql`
SELECT
    food_type,
    id AS HelloWorld,
    id AS hello_world
FROM items;
`
"#,

//// Generated TS interfaces ////
""
);


#[rustfmt::skip]
run_test!(retain_original2, TestConfig::new("postgres", true, Some(".sqlxrc.enabled.json".to_string())),
//// TS query ////
r#"
const someQuery = sql`
SELECT
    food_type,
    id AS HelloWorld,
    id AS hello_world
FROM items;
`
"#,

//// Generated TS interfaces ////
r#"asdasd"#
);

}
