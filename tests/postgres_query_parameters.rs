#[cfg(test)]
mod postgres_query_paramters_tests {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::env;
    use std::fs;
    use std::io::Write;
    use std::process::Command;
    use tempfile::tempdir;

    use pretty_assertions::assert_eq;
    use test_utils::test_utils::TSString;
    use test_utils::{run_test, sandbox::TestConfig};

    #[rustfmt::skip]
run_test!(should_pick_query_params_from_flat_list_of_binary_ops, TestConfig::new("postgres", None),

//// TS query ////
r#"
const someQuery = sql`
SELECT *
FROM items
WHERE points > $1
AND points < $2
OR points = $3
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
}
