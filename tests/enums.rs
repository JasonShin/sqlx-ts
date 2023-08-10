#[cfg(test)]
mod process_enum {
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
run_test!(should_check_and_generate_types_against_enum_field, TestConfig::new("mysql"),

//// TS query ////
r#"
const enumQuery = sql`
SELECT enum1
FROM random
`
"#,

//// Generated TS interfaces ////
r#"
export type EnumQueryParams = [];

export interface IEnumQueryResult {
    
};

export interface IEnumQueryQuery {
    params: EnumQueryParams;
    result: IEnumQueryResult;
};
"#);
}
