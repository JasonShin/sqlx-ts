#[cfg(test)]
mod postgres_query_parameters_tests {
  use std::env;
  use std::fs;
  use std::io::Write;
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

  // Issue #266: Query params should be detected when placeholder is on the left side of a comparison
  #[rustfmt::skip]
run_test!(should_pick_query_params_when_placeholder_before_comparison, TestConfig::new("postgres", true, None, None),

//// TS query ////
r#"
const someQuery = sql`
SELECT *
FROM inventory
WHERE quantity > 0
AND $1 >= quantity
AND quantity >= $2
`;
"#,

//// Generated TS interfaces ////
r#"
export type SomeQueryParams = [number | null, number | null];

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

  // Issue #266: Query params should be detected in BETWEEN clause when placeholder is the expr
  #[rustfmt::skip]
run_test!(should_pick_query_params_from_between_with_placeholder_expr, TestConfig::new("postgres", true, None, None),

//// TS query ////
r#"
const someQuery = sql`
SELECT *
FROM inventory
WHERE $1 BETWEEN quantity AND quantity
`;
"#,

//// Generated TS interfaces ////
r#"
export type SomeQueryParams = [number | null];

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

  // Issue #266: Standard BETWEEN with placeholders as low/high should still work
  #[rustfmt::skip]
run_test!(should_pick_query_params_from_between_with_placeholder_bounds, TestConfig::new("postgres", true, None, None),

//// TS query ////
r#"
const someQuery = sql`
SELECT *
FROM inventory
WHERE quantity BETWEEN $1 AND $2
`;
"#,

//// Generated TS interfaces ////
r#"
export type SomeQueryParams = [number | null, number | null];

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
