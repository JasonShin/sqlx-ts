#[cfg(test)]
mod mysql_insert_query_parameters {
  use assert_cmd::prelude::*;
  use pretty_assertions::assert_eq;
  use std::env;
  use std::fs;
  use std::io::Write;
  use std::process::Command;
  use tempfile::tempdir;

  use test_utils::test_utils::TSString;
  use test_utils::{run_test, sandbox::TestConfig};

  #[rustfmt::skip]
run_test!(should_pick_query_params_from_single_row_of_values, TestConfig::new("mysql", true, None, None),

//// TS query ////
r#"
const someInputQuery = sql`
INSERT INTO items (id, name, rarity, flavor_text)
VALUES
(?, ?, 'epic', 'asd');
`
"#,

//// Generated TS interfaces ////
r#"
export type SomeInputQueryParams = [[number, string]];

export interface ISomeInputQueryResult {
    
}

export interface ISomeInputQueryQuery {
    params: SomeInputQueryParams;
    result: ISomeInputQueryResult;
}
"#);

  #[rustfmt::skip]
run_test!(should_pick_query_params_from_multiple_rows_of_values, TestConfig::new("mysql", true, None, None),

//// TS query ////
r#"
import { sql } from "sqlx-ts";

const someInputQuery = sql`
INSERT INTO items (id, name, rarity, flavor_text)
VALUES
(?, ?, 'epic', 'test'),
(1, 'test', ?, ?);
`
"#,

//// Generated TS interfaces ////
r#"
export type SomeInputQueryParams = [[number, string], [string, string]];

export interface ISomeInputQueryResult {
    
}

export interface ISomeInputQueryQuery {
    params: SomeInputQueryParams;
    result: ISomeInputQueryResult;
}
"#);
}
