#[cfg(test)]
mod postgres_batch_insert {
  use assert_cmd::prelude::*;
  use std::env;
  use std::fs;
  use std::io::Write;
  use std::process::Command;
  use tempfile::tempdir;
  use test_utils::test_utils::TSString;
  use test_utils::{run_test, sandbox::TestConfig};

  #[rustfmt::skip]
run_test!(should_support_jsonb_to_recordset_batch_insert, TestConfig::new("postgres", true, None, None),

//// TS query ////
r#"
const batchInsertQuery = sql`
INSERT INTO items (id, name)
SELECT
  id,
  name
FROM jsonb_to_recordset($1) AS t(
  id INT,
  name TEXT
)
`
"#,

//// Generated TS interfaces ////
r#"
export type BatchInsertQueryParams = [any];

export interface IBatchInsertQueryResult {

}

export interface IBatchInsertQueryQuery {
    params: BatchInsertQueryParams;
    result: IBatchInsertQueryResult;
}
"#);

  #[rustfmt::skip]
run_test!(should_support_jsonb_to_recordset_with_multiple_columns, TestConfig::new("postgres", true, None, None),

//// TS query ////
r#"
const batchInsertMultipleQuery = sql`
INSERT INTO items (id, name, rarity, flavor_text)
SELECT
  id,
  name,
  rarity,
  flavor_text
FROM jsonb_to_recordset($1) AS t(
  id INT,
  name VARCHAR(100),
  rarity VARCHAR(50),
  flavor_text TEXT
)
`
"#,

//// Generated TS interfaces ////
r#"
export type BatchInsertMultipleQueryParams = [any];

export interface IBatchInsertMultipleQueryResult {

}

export interface IBatchInsertMultipleQueryQuery {
    params: BatchInsertMultipleQueryParams;
    result: IBatchInsertMultipleQueryResult;
}
"#);
}
