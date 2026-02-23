#[cfg(test)]
mod sets {
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
run_test!(should_generate_sets_for_mysql, TestConfig::new("mysql", true, None, None),

//// TS query ////
r#"
const setQuery = sql`
  SELECT
    set1
  FROM
    random
`;
"#,

//// Generated TS interfaces ////
r#"
export type SetQueryParams = [];

export interface ISetQueryResult {
    set1: string | null;
}

export interface ISetQueryQuery {
    params: SetQueryParams;
    result: ISetQueryResult;
}
"#);

  #[rustfmt::skip]
run_test!(should_generate_set_insert_params_for_mysql, TestConfig::new("mysql", true, None, None),

//// TS query ////
r#"
const setQuery = sql`
  INSERT INTO
    random (set1)
  VALUES
    (?)
`;
"#,

//// Generated TS interfaces ////
r#"
export type SetQueryParams = [[string | null]];

export interface ISetQueryResult {

}

export interface ISetQueryQuery {
    params: SetQueryParams;
    result: ISetQueryResult;
}
"#);
}
