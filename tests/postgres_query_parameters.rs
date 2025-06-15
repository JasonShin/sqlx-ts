#[cfg(test)]
mod postgres_query_parameters_tests {
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
run_test!(should_pick_query_params_from_flat_list_of_binary_ops, TestConfig::new("postgres", true, None, None),

//// TS query ////
r#"
const someQuery = sql`
SELECT *
FROM inventory
WHERE quantity > $1
AND quantity < $2
OR quantity = $3
`;
"#,

//// Generated TS interfaces ////
r#"
export type SomeQueryParams = [number | null, number | null, number | null];

export interface ISomeQueryResult {
    character_id: number | null;
    id: number;
    quantity: number | null;
}

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
}
"#);
}
