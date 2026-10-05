#[cfg(test)]
mod qualified_table_names_tests {
  use std::env;
  use std::fs;
  use std::io::Write;
  use tempfile::tempdir;

  use pretty_assertions::assert_eq;
  use test_utils::test_utils::TSString;
  use test_utils::{run_test, sandbox::TestConfig};

  #[rustfmt::skip]
run_test!(mysql_should_resolve_table_in_another_database, TestConfig::new("mysql", true, None, None),
//// TS query ////
r#"
const someQuery = sql`SELECT message, priority FROM staff.announcements WHERE message = ?`
"#,

//// Generated TS interfaces ////
r#"
export type SomeQueryParams = [string | null];

export interface ISomeQueryResult {
	message: string | null;
	priority: 'low' | 'high';
}

export interface ISomeQueryQuery {
	params: SomeQueryParams;
	result: ISomeQueryResult;
}
"#);

  #[rustfmt::skip]
run_test!(mysql_should_resolve_aliased_table_in_another_database_with_join, TestConfig::new("mysql", true, None, None),
//// TS query ////
r#"
const someQuery = sql`
SELECT a.message AS message, i.name AS item_name
FROM staff.announcements a
INNER JOIN items i ON i.id = a.id
WHERE a.priority = ?
`
"#,

//// Generated TS interfaces ////
r#"
export type SomeQueryParams = ['low' | 'high'];

export interface ISomeQueryResult {
	item_name: string;
	message: string | null;
}

export interface ISomeQueryQuery {
	params: SomeQueryParams;
	result: ISomeQueryResult;
}
"#);

  #[rustfmt::skip]
run_test!(mysql_should_resolve_insert_into_another_database, TestConfig::new("mysql", true, None, None),
//// TS query ////
r#"
const someQuery = sql`INSERT INTO staff.announcements (message, priority) VALUES (?, ?)`
"#,

//// Generated TS interfaces ////
r#"
export type SomeQueryParams = [[string | null, 'low' | 'high']];

export interface ISomeQueryResult {

}

export interface ISomeQueryQuery {
	params: SomeQueryParams;
	result: ISomeQueryResult;
}
"#);

  #[rustfmt::skip]
run_test!(postgres_should_resolve_table_in_another_schema, TestConfig::new("postgres", true, None, None),
//// TS query ////
r#"
const someQuery = sql`SELECT message, priority FROM staff.announcements WHERE message = $1`
"#,

//// Generated TS interfaces ////
r#"
export type SomeQueryParams = [string | null];

export interface ISomeQueryResult {
	message: string | null;
	priority: 'low' | 'high';
}

export interface ISomeQueryQuery {
	params: SomeQueryParams;
	result: ISomeQueryResult;
}
"#);

  #[rustfmt::skip]
run_test!(postgres_should_resolve_wildcard_in_another_schema, TestConfig::new("postgres", true, None, None),
//// TS query ////
r#"
const someQuery = sql`SELECT * FROM staff.announcements`
"#,

//// Generated TS interfaces ////
r#"
export type SomeQueryParams = [];

export interface ISomeQueryResult {
	id: number;
	message: string | null;
	priority: 'low' | 'high';
}

export interface ISomeQueryQuery {
	params: SomeQueryParams;
	result: ISomeQueryResult;
}
"#);

  #[rustfmt::skip]
run_test!(postgres_should_resolve_update_in_another_schema, TestConfig::new("postgres", true, None, None),
//// TS query ////
r#"
const someQuery = sql`UPDATE staff.announcements SET message = $1 WHERE priority = $2`
"#,

//// Generated TS interfaces ////
r#"
export type SomeQueryParams = [string | null, 'low' | 'high'];

export interface ISomeQueryResult {

}

export interface ISomeQueryQuery {
	params: SomeQueryParams;
	result: ISomeQueryResult;
}
"#);

  #[rustfmt::skip]
run_test!(postgres_should_still_resolve_public_schema_explicitly, TestConfig::new("postgres", true, None, None),
//// TS query ////
r#"
const someQuery = sql`SELECT name FROM public.items WHERE id = $1`
"#,

//// Generated TS interfaces ////
r#"
export type SomeQueryParams = [number];

export interface ISomeQueryResult {
	name: string;
}

export interface ISomeQueryQuery {
	params: SomeQueryParams;
	result: ISomeQueryResult;
}
"#);
}
