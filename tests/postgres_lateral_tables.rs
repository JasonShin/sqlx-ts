#[cfg(test)]
mod postgres_table_lateral_functions {
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
const jsonbToRecordSet1 = sql`
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
export type JsonbToRecordSet1Params = [any];

export interface IJsonbToRecordSet1Result {
	id: number;
	name: string;
}

export interface IJsonbToRecordSet1Query {
	params: JsonbToRecordSet1Params;
	result: IJsonbToRecordSet1Result;
}
"#);

  #[rustfmt::skip]
run_test!(should_support_jsonb_to_recordset_with_multiple_columns, TestConfig::new("postgres", true, None, None),

//// TS query ////
r#"
const jsonbToRecordSet12 = sql`
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
export type JsonbToRecordSet12Params = [any];

export interface IJsonbToRecordSet12Result {
	flavor_text: string;
	id: number;
	name: string;
	rarity: string;
}

export interface IJsonbToRecordSet12Query {
	params: JsonbToRecordSet12Params;
	result: IJsonbToRecordSet12Result;
}
"#);
}
