#[cfg(test)]
mod mysql_query_parameters_tests {
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
run_test!(should_pick_query_params_from_flat_list_of_binary_ops, TestConfig::new("mysql", true, None, None),

//// TS query ////
r#"
const someQuery = sql`
SELECT *
FROM items
WHERE name = ?
AND rarity = ?
`;
"#,

//// Generated TS interfaces ////
r#"
export type SomeQueryParams = [string, string | null];

export interface ISomeQueryResult {
    flavor_text: string | null;
    id: number;
    inventory_id: number | null;
    name: string;
    rarity: string | null;
};

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
};
"#);

  #[rustfmt::skip]
run_test!(should_pick_query_params_from_in_list, TestConfig::new("mysql", true, None, None),

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
    flavor_text: string | null;
    id: number;
    inventory_id: number | null;
    name: string;
    rarity: string | null;
};

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
};
"#);

  #[rustfmt::skip]
run_test!(should_pick_query_params_from_in_subqueries, TestConfig::new("mysql", true,  None, None),

//// TS query ////
r#"
const someQuery = sql`
SELECT *
FROM items
WHERE id IN (
    SELECT id
    FROM items
    WHERE name = ?
    AND id IN (
        SELECT id
        FROM items
        WHERE rarity = ?
    )
) AND id < ?;
`;
"#,

//// Generated TS interfaces ////
r#"
export type SomeQueryParams = [string, string | null, number];

export interface ISomeQueryResult {
    flavor_text: string | null;
    id: number;
    inventory_id: number | null;
    name: string;
    rarity: string | null;
};

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
};
"#);

  #[rustfmt::skip]
run_test!(should_pick_query_params_from_subqueries, TestConfig::new("mysql", true, None, None),

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
        WHERE rarity = ?
    )
) AND rarity = ?;
`;
"#,

//// Generated TS interfaces ////
r#"
export type SomeQueryParams = [number, string | null, string | null];

export interface ISomeQueryResult {
    flavor_text: string | null;
    id: number;
    inventory_id: number | null;
    name: string;
    rarity: string | null;
};

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
};
"#);
}
