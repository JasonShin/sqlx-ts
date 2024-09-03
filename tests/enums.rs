#[cfg(test)]
mod enums {
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
run_test!(should_generate_enums_for_mysql, TestConfig::new("mysql", true, None, None),

//// TS query ////
r#"
const usersQuery = sql`
  SELECT
    enum1
  FROM
    random
`;
"#,

//// Generated TS interfaces ////
r#"
export type UsersQueryParams = [];

export interface IUsersQueryResult {
    enum1: 'x-small' | 'small' | 'medium' | 'large' | 'x-large';
};

export interface IUsersQueryQuery {
    params: UsersQueryParams;
    result: IUsersQueryResult;
};
"#);

  #[rustfmt::skip]
run_test!(should_generate_enums_for_postgres, TestConfig::new("postgres", true, None, None),

//// TS query ////
r#"
const usersQuery = sql`
  SELECT
    enum1
  FROM
    random
`;
"#,

//// Generated TS interfaces ////
r#"
export type UsersQueryParams = [];

export interface IUsersQueryResult {
    enum1: 'x-small' | 'small' | 'medium' | 'large' | 'x-large';
};

export interface IUsersQueryQuery {
    params: UsersQueryParams;
    result: IUsersQueryResult;
};
"#);
}
