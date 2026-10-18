#[cfg(test)]
mod postgres_update_query_parameters {
  use pretty_assertions::assert_eq;
  use std::env;
  use std::fs;
  use std::io::Write;
  use tempfile::tempdir;

  use test_utils::test_utils::TSString;
  use test_utils::{run_test, sandbox::TestConfig};

  #[rustfmt::skip]
run_test!(should_pick_query_params_from_single_row_of_values, TestConfig::new("postgres", true, None, None),

//// TS query ////
r#"
const someUpdateQuery = sql`
UPDATE items
SET rarity = $1;
`
"#,

//// Generated TS interfaces ////
r#"
export type SomeUpdateQueryParams = [string | null];

export interface ISomeUpdateQueryResult {
    
}

export interface ISomeUpdateQueryQuery {
    params: SomeUpdateQueryParams;
    result: ISomeUpdateQueryResult;
}
"#);

  #[rustfmt::skip]
run_test!(should_pick_query_params_from_arithmetic_in_set, TestConfig::new("postgres", true, None, None),

//// TS query ////
r#"
const someUpdateQuery = sql`
UPDATE characters
SET gold = gold + ($2 * 86400)
WHERE name = $1
`
"#,

//// Generated TS interfaces ////
r#"
export type SomeUpdateQueryParams = [string, number | null];

export interface ISomeUpdateQueryResult {
    
}

export interface ISomeUpdateQueryQuery {
    params: SomeUpdateQueryParams;
    result: ISomeUpdateQueryResult;
}
"#);
}
