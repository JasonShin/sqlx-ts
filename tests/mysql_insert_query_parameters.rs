#[cfg(test)]
mod mysql_insert_query_parameters {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use pretty_assertions::assert_eq;
    use std::fs;
    use std::io::Write;
    use std::process::Command;
    use tempfile::tempdir;

    use test_utils::test_utils::TSString;
    use test_utils::{run_test, sandbox::TestConfig};

    #[rustfmt::skip]
run_test!(should_pick_query_params_from_single_row_of_values, TestConfig::new("mysql"),

//// TS query ////
r#"
const someInputQuery = sql`
INSERT INTO items (id, food_type, time_takes_to_cook, table_id, points)
VALUES
(?, ?, 2, 1, 2);
`
"#,

//// Generated TS interfaces ////
r#"
export type SomeInputQueryParams = [[number, string]];

export interface ISomeInputQueryResult {
    
};

export interface ISomeInputQueryQuery {
    params: SomeInputQueryParams;
    result: ISomeInputQueryResult;
};
"#);

    #[rustfmt::skip]
run_test!(should_pick_query_params_from_multiple_rows_of_values, TestConfig::new("mysql"),

//// TS query ////
r#"
import { sql } from "sqlx-ts";

const someInputQuery = sql`
INSERT INTO items (id, food_type, time_takes_to_cook, table_id, points)
VALUES
(?, ?, 2, 1, 2),
(1, 'test', ?, ?, ?);
`
"#,

//// Generated TS interfaces ////
r#"
export type SomeInputQueryParams = [[number, string], [number, number, number]];

export interface ISomeInputQueryResult {
    
};

export interface ISomeInputQueryQuery {
    params: SomeInputQueryParams;
    result: ISomeInputQueryResult;
};
"#);
}
