#[cfg(test)]
mod mysql_delete_query_parameters {
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
run_test!(should_pick_query_params_from_binary_ops, TestConfig::new("postgres", true, None, None),

//// TS query ////
r#"
const someDeleteQuery = sql`
DELETE FROM inventory
WHERE id = $2
AND quantity > 1
OR character_id = $1;
`
"#,

//// Generated TS interfaces ////
r#"
export type SomeDeleteQueryParams = [number | null, number];

export interface ISomeDeleteQueryResult {
    
}

export interface ISomeDeleteQueryQuery {
    params: SomeDeleteQueryParams;
    result: ISomeDeleteQueryResult;
}
"#);

  #[rustfmt::skip]
run_test!(should_pick_query_params_from_subquery, TestConfig::new("postgres", true, None, None),

//// TS query ////
r#"
const someDeleteQuery = sql`
DELETE FROM inventory
WHERE id = $2
AND quantity > 1
OR character_id = (SELECT id FROM characters WHERE id = $1);
`
"#,

//// Generated TS interfaces ////
r#"
export type SomeDeleteQueryParams = [number, number];

export interface ISomeDeleteQueryResult {
    
}

export interface ISomeDeleteQueryQuery {
    params: SomeDeleteQueryParams;
    result: ISomeDeleteQueryResult;
}
"#);
}
