/// Test suites for converting any case to a naming convention if `columnNamingConvention` is provided
///
#[cfg(test)]
mod column_naming_convention_tests {
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
run_test!(camel_case, TestConfig::new("postgres", true, None, Some(".sqlxrc.column_camel.json".to_string())),
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
    foodType: string;
    helloWorld: number;
};

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
};
"#
);

  #[rustfmt::skip]
run_test!(pascal_case, TestConfig::new("postgres", true, None, Some(".sqlxrc.column_pascal.json".to_string())),

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
    FoodType: string;
    HelloWorld1: number;
    HelloWorld2: number;
};

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
};
"#
);

  #[rustfmt::skip]
run_test!(pascal_and_ignore_convert_to_camel_case, TestConfig::new("postgres", true, None, Some(".sqlxrc.column_pascal_override.json".to_string())),

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
    FoodType: string;
    HelloWorld1: number;
    HelloWorld2: number;
};

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
};
"#
);
}
