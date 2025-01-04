/// Test suites for converting any case to camelCase if generateTypes.convertToCamelCase is true
///
#[cfg(test)]
mod convert_camelcase_tests {
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
run_test!(retain_original, TestConfig::new("postgres", true, None, Some(".sqlxrc.camelcase1.json".to_string())),
//// TS query ////
r#"
const someQuery = sql`
SELECT
    inventory_id,
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
    hello_world: number;
    inventory_id: number | null;
};

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
};
"#
);

  #[rustfmt::skip]
run_test!(convert_camelcase, TestConfig::new("postgres", true, None, Some(".sqlxrc.camelcase2.json".to_string())),

//// TS query ////
r#"
const someQuery = sql`
SELECT
    inventory_id,
    id AS HelloWorld1,
    id AS hello_world2
FROM items;
`
"#,

//// Generated TS interfaces ////
r#"
export type SomeQueryParams = [];

export interface ISomeQueryResult {
    helloWorld1: number;
    helloWorld2: number;
    inventoryId: number | null;
};

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
};
"#
);

  #[rustfmt::skip]
run_test!(retain_original_on_missing_config, TestConfig::new("postgres", true, None, Some(".sqlxrc.camelcase3.json".to_string())),

//// TS query ////
r#"
const someQuery = sql`
SELECT
    inventory_id,
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
    hello_world2: number;
    inventory_id: number | null;
};

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
};
"#
);
}
