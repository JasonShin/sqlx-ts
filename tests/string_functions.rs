#[cfg(test)]
mod string_functions_tests {
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
run_test!(overlay, TestConfig::new("postgres", None),

//// TS query ////
r#"
const someQuery = sql`
SELECT
    OVERLAY($1 PLACING $2 FROM $3) AS test
FROM items;
`
"#,

//// Generated TS interfaces ////
r#"
export type SomeQueryParams = [string, string, number];

export interface ISomeQueryResult {
    test: string;
};

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
};
"#
);

    #[rustfmt::skip]
run_test!(trim, TestConfig::new("postgres", None),

//// TS query ////
"const someQuery = sql`SELECT TRIM($1) AS test FROM items;`",

//// Generated TS interfaces ////
r#"
export type SomeQueryParams = [string];

export interface ISomeQueryResult {
    test: string;
};

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
};
"#
);

    #[rustfmt::skip]
run_test!(substring, TestConfig::new("postgres", None),

//// TS query ////"
"const someQuery = sql`SELECT SUBSTRING($1, 5, 6) AS ExtractString FROM items;`",

//// Generated TS interfaces ////
r#"
export type SomeQueryParams = [string];

export interface ISomeQueryResult {
    ExtractString: string;
};

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
};
"#
);

    #[rustfmt::skip]
run_test!(like, TestConfig::new("postgres", None),

//// TS query ////"
"const someQuery = sql`SELECT id FROM items WHERE food_type LIKE $1;`",

//// Generated TS interfaces ////
r#"
export type SomeQueryParams = [string];

export interface ISomeQueryResult {
    id: number;
};

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
};
"#
);
}
