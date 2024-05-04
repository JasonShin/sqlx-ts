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
run_test!(retain_original, TestConfig::new("postgres", Some(".sqlxrc.camelcase1.json".to_string())),
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
r#"
export type SomeQueryParams = [];

export interface ISomeQueryResult {
    HelloWorld: number;
    food_type: string;
    hello_world: number;
};

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
};
"#
);

    #[rustfmt::skip]
run_test!(convert_camelcase, TestConfig::new("postgres", Some(".sqlxrc.camelcase2.json".to_string())),

//// TS query ////
r#"
const someQuery = sql`
SELECT
    food_type,
    id AS HelloWorld1,
    id AS hello_world2
FROM items;
`
"#,

//// Generated TS interfaces ////
r#"
export type SomeQueryParams = [];

export interface ISomeQueryResult {
    foodType: string;
    helloWorld1: number;
    helloWorld2: number;
};

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
};
"#
);


    #[rustfmt::skip]
run_test!(retain_original_on_missing_config, TestConfig::new("postgres", Some(".sqlxrc.camelcase3.json".to_string())),

//// TS query ////
r#"
const someQuery = sql`
SELECT
    food_type,
    id AS HelloWorld1,
    id AS hello_world2
FROM items;
`
"#,

//// Generated TS interfaces ////
r#"
export type SomeQueryParams = [];

export interface ISomeQueryResult {
    HelloWorld1: number;
    food_type: string;
    hello_world2: number;
};

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
};
"#
);
}
