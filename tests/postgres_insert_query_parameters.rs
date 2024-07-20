#[cfg(test)]
mod postgres_insert_query_parameters {
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
run_test!(should_pick_query_params_from_single_row_of_values, TestConfig::new("postgres", true, None, None),

//// TS query ////
r#"
const someInputQuery = sql`
INSERT INTO items (id, "food_type", time_takes_to_cook, table_id, points)
VALUES
($2, $1, 2, $3, 2);
`
"#,

//// Generated TS interfaces ////
r#"
export type SomeInputQueryParams = [string, number, number];

export interface ISomeInputQueryResult {
    
};

export interface ISomeInputQueryQuery {
    params: SomeInputQueryParams;
    result: ISomeInputQueryResult;
};
"#);

  #[rustfmt::skip]
run_test!(should_pick_query_params_from_multiple_rows_of_values, TestConfig::new("postgres", true, None, None),

//// TS query ////
r#"
const someInputQuery = sql`
INSERT INTO items (id, "food_type", time_takes_to_cook, table_id, points)
VALUES
($2, $1, 2, $3, 2),
($5, 'test', $4, $7, $6);
`
"#,

//// Generated TS interfaces ////
r#"
export type SomeInputQueryParams = [string, number, number, number, number, number, number];

export interface ISomeInputQueryResult {
    
};

export interface ISomeInputQueryQuery {
    params: SomeInputQueryParams;
    result: ISomeInputQueryResult;
};
"#);
}
