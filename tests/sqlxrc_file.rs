/// Test suites for converting any case to camelCase if generateTypes.convertToCamelCase is true
///
#[cfg(test)]
mod sqlxrc_file {
  use assert_cmd::prelude::*;
  use std::env;
  use std::fs;
  use std::io::Write;
  use std::process::Command;
  use tempfile::tempdir;

  use pretty_assertions::assert_eq;
  use test_utils::test_utils::TSString;
  use test_utils::{run_test, sandbox::TestConfig};

  #[rustfmt::skip]
run_test!(not_enabled, TestConfig::new("postgres", false, None, Some(".sqlxrc.not_enabled.json".to_string())),
//// TS query ////
r#"
const someQuery = sql`
SELECT
    name,
    id AS HelloWorld,
    id AS hello_world
FROM items;
`
"#,

//// Generated TS interfaces ////
""
);

  #[rustfmt::skip]
run_test!(not_enabled_but_enabled_cli, TestConfig::new("postgres", true, None, Some(".sqlxrc.not_enabled.json".to_string())),
//// TS query ////
r#"
const someQuery = sql`
SELECT
    name,
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
    name: string;
}

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
}
"#
);

  #[rustfmt::skip]
run_test!(enabled_and_enabled_cli, TestConfig::new("postgres", true, None, Some(".sqlxrc.enabled.json".to_string())),
//// TS query ////
r#"
const someQuery = sql`
SELECT
    name,
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
    name: string;
}

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
}
"#
);

  #[rustfmt::skip]
run_test!(enabled_but_not_enabled_cli, TestConfig::new("postgres", false, None, Some(".sqlxrc.enabled.json".to_string())),
//// TS query ////
r#"
const someQuery = sql`
SELECT
    name,
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
    name: string;
}

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
}
"#
);
}
