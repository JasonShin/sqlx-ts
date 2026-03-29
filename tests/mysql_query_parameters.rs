#[cfg(test)]
mod mysql_query_parameters_tests {
  use pretty_assertions::assert_eq;
  use std::env;
  use std::fs;
  use std::io::Write;
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
}

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
}
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
}

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
}
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
}

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
}
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
}

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
}
"#);

  // Issue #266: Query params should be detected when placeholder is on the left side of a comparison
  // Skipped on MySQL <= 5.7 because TIMESTAMP nullability defaults differ between versions
  #[rustfmt::skip]
run_test!(should_pick_query_params_when_placeholder_before_comparison, TestConfig::new("mysql", true, None, None),

//// TS query ////
r#"
const someQuery = sql`
SELECT *
FROM characters
WHERE level = 1
AND ? >= login_time
AND logout_time >= ?
`;
"#,

//// Generated TS interfaces ////
r#"
export type SomeQueryParams = [Date | null, Date | null];

export interface ISomeQueryResult {
    class_id: number | null;
    created_at: Date | null;
    experience: number | null;
    gold: number | null;
    id: number;
    last_chat_time: unknown | null;
    last_trade_time: unknown | null;
    level: number | null;
    login_time: Date | null;
    logout_time: Date | null;
    name: string;
    race_id: number | null;
}

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
}
"#, min_mysql: (5, 7));

  // Issue #266: Query params should be detected in BETWEEN clause when placeholder is the expr
  // Skipped on MySQL <= 5.7 because TIMESTAMP nullability defaults differ between versions
  #[rustfmt::skip]
run_test!(should_pick_query_params_from_between_with_placeholder_expr, TestConfig::new("mysql", true, None, None),

//// TS query ////
r#"
const someQuery = sql`
SELECT *
FROM characters
WHERE level = 1
AND ? BETWEEN login_time AND logout_time
`;
"#,

//// Generated TS interfaces ////
r#"
export type SomeQueryParams = [Date | null];

export interface ISomeQueryResult {
    class_id: number | null;
    created_at: Date | null;
    experience: number | null;
    gold: number | null;
    id: number;
    last_chat_time: unknown | null;
    last_trade_time: unknown | null;
    level: number | null;
    login_time: Date | null;
    logout_time: Date | null;
    name: string;
    race_id: number | null;
}

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
}
"#, min_mysql: (5, 7));

  // Issue #266: Standard BETWEEN with placeholders as low/high should still work
  // Skipped on MySQL <= 5.7 because TIMESTAMP nullability defaults differ between versions
  #[rustfmt::skip]
run_test!(should_pick_query_params_from_between_with_placeholder_bounds, TestConfig::new("mysql", true, None, None),

//// TS query ////
r#"
const someQuery = sql`
SELECT *
FROM characters
WHERE login_time BETWEEN ? AND ?
`;
"#,

//// Generated TS interfaces ////
r#"
export type SomeQueryParams = [Date | null, Date | null];

export interface ISomeQueryResult {
    class_id: number | null;
    created_at: Date | null;
    experience: number | null;
    gold: number | null;
    id: number;
    last_chat_time: unknown | null;
    last_trade_time: unknown | null;
    level: number | null;
    login_time: Date | null;
    logout_time: Date | null;
    name: string;
    race_id: number | null;
}

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
}
"#, min_mysql: (5, 7));
}
