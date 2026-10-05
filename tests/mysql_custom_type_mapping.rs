#[cfg(test)]
mod mysql_custom_type_mapping_tests {
  use std::env;
  use std::fs;
  use std::io::Write;
  use tempfile::tempdir;

  use pretty_assertions::assert_eq;
  use test_utils::test_utils::TSString;
  use test_utils::{run_test, sandbox::TestConfig};

  #[rustfmt::skip]
run_test!(should_prefer_column_type_over_data_type, TestConfig::new("mysql", true, None, Some(".sqlxrc.type_mapping_mysql.json".to_string())),
//// TS query ////
r#"
const someQuery = sql`
SELECT id, is_active, flags, big_id, amount
FROM type_mapping_test
WHERE big_id = ? AND is_active = ?
`
"#,

//// Generated TS interfaces ////
r#"
export type SomeQueryParams = [string, boolean];

export interface ISomeQueryResult {
	amount: bigint;
	big_id: string;
	flags: number;
	id: number;
	is_active: boolean;
}

export interface ISomeQueryQuery {
	params: SomeQueryParams;
	result: ISomeQueryResult;
}
"#);
}
