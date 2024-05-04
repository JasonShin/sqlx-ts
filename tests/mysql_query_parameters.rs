#[cfg(test)]
mod mysql_query_parameters_tests {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use pretty_assertions::assert_eq;
    use std::env;
    use std::fs;
    use std::io::Write;
    use std::process::Command;
    use tempfile::tempdir;

    use test_utils::test_utils::TSString;
    use test_utils::{run_test, sandbox::TestConfig};

    #[rustfmt::skip]
run_test!(should_pick_query_params_from_flat_list_of_binary_ops, TestConfig::new("mysql", None),

//// TS query ////
r#"
const someQuery = sql`
SELECT *
FROM items
WHERE points > ?
AND points < ?
OR points = ?
`;
"#,

//// Generated TS interfaces ////
r#"
export type SomeQueryParams = [number, number, number];

export interface ISomeQueryResult {
    food_type: string;
    id: number;
    points: number;
    table_id: number;
    time_takes_to_cook: number;
};

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
};
"#);

    #[rustfmt::skip]
run_test!(should_pick_query_params_from_in_list, TestConfig::new("mysql", None),

//// TS query ////
r#"
const someQuery = sql`
SELECT *
FROM items
WHERE id IN (?);
`
"#,

//// Generated TS interfaces ////
r#"
export type SomeQueryParams = [Array<number>];

export interface ISomeQueryResult {
    food_type: string;
    id: number;
    points: number;
    table_id: number;
    time_takes_to_cook: number;
};

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
};
"#);

    #[rustfmt::skip]
run_test!(should_pick_query_params_from_in_subqueries, TestConfig::new("mysql", None),

//// TS query ////
r#"
const someQuery = sql`
SELECT *
FROM items
WHERE id IN (
    SELECT id
    FROM items
    WHERE points > ?
    AND id IN (
        SELECT id
        FROM items
        WHERE food_type = ?
    )
) AND points < ?;
`;
"#,

//// Generated TS interfaces ////
r#"
export type SomeQueryParams = [number, string, number];

export interface ISomeQueryResult {
    food_type: string;
    id: number;
    points: number;
    table_id: number;
    time_takes_to_cook: number;
};

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
};
"#);

    #[rustfmt::skip]
run_test!(should_pick_query_params_from_subqueries, TestConfig::new("mysql", None),

//// TS query ////
r#"
const someQuery = sql`
SELECT *
FROM items
WHERE id = (
    SELECT id
    FROM items
    WHERE id = ?
    AND id = (
        SELECT id
        FROM items
        WHERE food_type = ?
    )
) AND food_type = ?;
`;
"#,

//// Generated TS interfaces ////
r#"
export type SomeQueryParams = [number, string, string];

export interface ISomeQueryResult {
    food_type: string;
    id: number;
    points: number;
    table_id: number;
    time_takes_to_cook: number;
};

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
};
"#);
}
