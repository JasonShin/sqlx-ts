#[cfg(test)]
mod string_functions_tests {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::fs;
    use std::io::Write;
    use std::process::Command;
    use tempfile::tempdir;

    use pretty_assertions::assert_eq;
    use test_utils::test_utils::TSString;
    use test_utils::{run_test, sandbox::TestConfig};

    #[rustfmt::skip]
run_test!(camelcase, TestConfig::new("postgres"),

//// TS query ////
r#"
const someQuery = sql`
SELECT
    id AS hello_world1,
    id AS helloWorld2,
    id AS HelloWorld3
FROM items;
`
"#,

//// Generated TS interfaces ////
r#"
export type SomeQueryParams = [];

export interface ISomeQueryResult {
    helloWorld1: number;
    helloWorld2: number;
    helloWorld3: number;
};

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
};
"#
);

}
