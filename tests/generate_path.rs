/// Test suites for converting any case to camelCase if generateTypes.convertToCamelCase is true
///
#[cfg(test)]
mod generate_path_tests {
  use assert_cmd::prelude::*;
  use std::env;
  use std::fs;
  use std::io::Write;
  use std::path::PathBuf;
  use std::process::Command;
  use tempfile::tempdir;

  use pretty_assertions::assert_eq;
  use test_utils::test_utils::TSString;
  use test_utils::{run_test, sandbox::TestConfig};

  #[rustfmt::skip]
run_test!(should_generate_path, TestConfig::new("postgres", true, Some(PathBuf::from("types/types.ts")), Some(".sqlxrc.camelcase1.json".to_string())),
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
}
