#[cfg(test)]
mod custom_type_mapping_tests {
  use std::fs;
  use std::io::Write;
  use tempfile::tempdir;

  use assert_cmd::cargo::cargo_bin_cmd;
  use pretty_assertions::assert_eq;
  use test_utils::test_utils::TSString;

  fn run_type_mapping_test(
    schema_sql: &str,
    ts_content: &str,
    type_mapping_json: &str,
  ) -> Result<(String, String), Box<dyn std::error::Error>> {
    run_type_mapping_test_with_files(schema_sql, &[("index.ts", ts_content)], type_mapping_json, None)
  }

  fn run_type_mapping_test_with_files(
    schema_sql: &str,
    ts_files: &[(&str, &str)],
    type_mapping_json: &str,
    generate_path: Option<&str>,
  ) -> Result<(String, String), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let parent_path = dir.path();

    let db_path = parent_path.join("test.db");
    let conn = rusqlite::Connection::open(&db_path)?;
    conn.execute_batch(schema_sql)?;
    drop(conn);

    let config = format!(
      r#"{{
  "generate_types": {{
    "enabled": true
  }},
  "connections": {{
    "default": {{
      "DB_TYPE": "sqlite",
      "DB_NAME": "{}",
      "type_mapping": {}
    }}
  }}
}}"#,
      db_path.display(),
      type_mapping_json
    );
    let config_path = parent_path.join(".sqlxrc.json");
    let mut config_file = fs::File::create(&config_path)?;
    write!(config_file, "{}", config)?;

    for (file_name, ts_content) in ts_files {
      let file_path = parent_path.join(file_name);
      let mut temp_file = fs::File::create(&file_path)?;
      writeln!(temp_file, "{}", ts_content)?;
    }

    let mut cmd = cargo_bin_cmd!("sqlx-ts");
    cmd
      .arg(parent_path.to_str().unwrap())
      .arg("--ext=ts")
      .arg("--db-type=sqlite")
      .arg(format!("--db-name={}", db_path.display()))
      .arg(format!("--config={}", config_path.display()))
      .arg("-g");
    if let Some(generate_path) = generate_path {
      cmd.arg(format!("--generate-path={}", parent_path.join(generate_path).display()));
    }

    let output = cmd.output()?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    assert!(
      output.status.success(),
      "sqlx-ts failed!\nstdout: {stdout}\nstderr: {stderr}"
    );

    let type_file_path = match generate_path {
      Some(generate_path) => parent_path.join(generate_path),
      None => parent_path.join("index.queries.ts"),
    };
    let type_file = if type_file_path.exists() {
      fs::read_to_string(type_file_path)?
    } else {
      String::new()
    };

    Ok((stdout, type_file))
  }

  #[test]
  fn should_override_integer_to_string() -> Result<(), Box<dyn std::error::Error>> {
    let schema = "CREATE TABLE test_custom_types (id INTEGER PRIMARY KEY NOT NULL, count BIGINT NOT NULL);";

    let ts_content = r#"
import { sql } from 'sqlx-ts'
const someQuery = sql`SELECT * FROM test_custom_types`
"#;

    let type_mapping = r#"{ "bigint": "string" }"#;

    let (_, type_file) = run_type_mapping_test(schema, ts_content, type_mapping)?;

    let expected = r#"
export type SomeQueryParams = [];

export interface ISomeQueryResult {
	count: string;
	id: number;
}

export interface ISomeQueryQuery {
	params: SomeQueryParams;
	result: ISomeQueryResult;
}
"#;

    assert_eq!(
      expected.trim().to_string().flatten(),
      type_file.trim().to_string().flatten()
    );
    Ok(())
  }

  #[test]
  fn should_override_with_union_type() -> Result<(), Box<dyn std::error::Error>> {
    let schema = "CREATE TABLE test_custom_types (id INTEGER PRIMARY KEY NOT NULL, count BIGINT NOT NULL);";

    let ts_content = r#"
import { sql } from 'sqlx-ts'
const someQuery = sql`SELECT * FROM test_custom_types`
"#;

    let type_mapping = r#"{ "bigint": "string | number" }"#;

    let (_, type_file) = run_type_mapping_test(schema, ts_content, type_mapping)?;

    let expected = r#"
export type SomeQueryParams = [];

export interface ISomeQueryResult {
	count: string | number;
	id: number;
}

export interface ISomeQueryQuery {
	params: SomeQueryParams;
	result: ISomeQueryResult;
}
"#;

    assert_eq!(
      expected.trim().to_string().flatten(),
      type_file.trim().to_string().flatten()
    );
    Ok(())
  }

  #[test]
  fn should_override_with_import() -> Result<(), Box<dyn std::error::Error>> {
    let schema = "CREATE TABLE events (id INTEGER PRIMARY KEY NOT NULL, created_at DATETIME NOT NULL);";

    let ts_content = r#"
import { sql } from 'sqlx-ts'
const someQuery = sql`SELECT * FROM events`
"#;

    let type_mapping = r#"{ "datetime": { "type": "DateTime", "import": "import type { DateTime } from \"luxon\"" } }"#;

    let (_, type_file) = run_type_mapping_test(schema, ts_content, type_mapping)?;

    assert!(
      type_file.contains("import type { DateTime } from \"luxon\""),
      "Expected import statement in generated file, got:\n{type_file}"
    );

    assert!(
      type_file.contains("created_at: DateTime;"),
      "Expected DateTime type for created_at, got:\n{type_file}"
    );

    Ok(())
  }

  #[test]
  fn should_not_override_unmapped_types() -> Result<(), Box<dyn std::error::Error>> {
    let schema =
      "CREATE TABLE test_custom_types (id INTEGER PRIMARY KEY NOT NULL, name TEXT NOT NULL, count BIGINT NOT NULL);";

    let ts_content = r#"
import { sql } from 'sqlx-ts'
const someQuery = sql`SELECT * FROM test_custom_types`
"#;

    let type_mapping = r#"{ "bigint": "string" }"#;

    let (_, type_file) = run_type_mapping_test(schema, ts_content, type_mapping)?;

    let expected = r#"
export type SomeQueryParams = [];

export interface ISomeQueryResult {
	count: string;
	id: number;
	name: string;
}

export interface ISomeQueryQuery {
	params: SomeQueryParams;
	result: ISomeQueryResult;
}
"#;

    assert_eq!(
      expected.trim().to_string().flatten(),
      type_file.trim().to_string().flatten()
    );
    Ok(())
  }

  #[test]
  fn should_keep_nullability_and_apply_to_params() -> Result<(), Box<dyn std::error::Error>> {
    let schema = "CREATE TABLE test_custom_types (id INTEGER PRIMARY KEY NOT NULL, count BIGINT);";

    let ts_content = r#"
import { sql } from 'sqlx-ts'
const someQuery = sql`SELECT * FROM test_custom_types WHERE count = ?`
"#;

    let type_mapping = r#"{ "BIGINT": "string" }"#;

    let (_, type_file) = run_type_mapping_test(schema, ts_content, type_mapping)?;

    let expected = r#"
export type SomeQueryParams = [string | null];

export interface ISomeQueryResult {
	count: string | null;
	id: number;
}

export interface ISomeQueryQuery {
	params: SomeQueryParams;
	result: ISomeQueryResult;
}
"#;

    assert_eq!(
      expected.trim().to_string().flatten(),
      type_file.trim().to_string().flatten()
    );
    Ok(())
  }

  #[test]
  fn should_write_import_once_with_generate_path() -> Result<(), Box<dyn std::error::Error>> {
    let schema = "CREATE TABLE events (id INTEGER PRIMARY KEY NOT NULL, created_at DATETIME NOT NULL);";

    let first = r#"
import { sql } from 'sqlx-ts'
const firstQuery = sql`SELECT * FROM events`
"#;
    let second = r#"
import { sql } from 'sqlx-ts'
const secondQuery = sql`SELECT created_at FROM events`
"#;

    let type_mapping = r#"{ "datetime": { "type": "DateTime", "import": "import type { DateTime } from \"luxon\"" } }"#;

    let (_, type_file) = run_type_mapping_test_with_files(
      schema,
      &[("first.ts", first), ("second.ts", second)],
      type_mapping,
      Some("types.ts"),
    )?;

    assert_eq!(
      type_file.matches("import type { DateTime } from \"luxon\";").count(),
      1,
      "Expected a single import statement in generated file, got:\n{type_file}"
    );
    assert!(
      type_file.starts_with("import type { DateTime } from \"luxon\";"),
      "Expected the import statement at the top of generated file, got:\n{type_file}"
    );
    assert_eq!(type_file.matches("created_at: DateTime;").count(), 2);
    Ok(())
  }

  const COMPLEX_SCHEMA: &str = "CREATE TABLE users (id INTEGER PRIMARY KEY NOT NULL, name VARCHAR(255) NOT NULL, balance DECIMAL(10,2) NOT NULL, created_at DATETIME NOT NULL, deleted_at DATETIME); CREATE TABLE orders (id INTEGER PRIMARY KEY NOT NULL, user_id INTEGER NOT NULL, total DECIMAL(10,2) NOT NULL, ordered_at DATETIME NOT NULL, shipped_at TIMESTAMP);";

  const COMPLEX_TYPE_MAPPING: &str = r#"{ "varchar": "UserName", "decimal": "string", "datetime": { "type": "DateTime", "import": "import type { DateTime } from \"luxon\"" }, "timestamp": { "type": "Dayjs", "import": "import type { Dayjs } from \"dayjs\"" } }"#;

  fn assert_complex_query_types(ts_content: &str, expected: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (_, type_file) = run_type_mapping_test(COMPLEX_SCHEMA, ts_content, COMPLEX_TYPE_MAPPING)?;
    assert_eq!(
      expected.trim().to_string().flatten(),
      type_file.trim().to_string().flatten()
    );
    Ok(())
  }

  #[test]
  fn should_map_types_across_joins() -> Result<(), Box<dyn std::error::Error>> {
    assert_complex_query_types(
      r#"
import { sql } from 'sqlx-ts'
const joinQuery = sql`SELECT u.id AS id, u.name AS name, o.total AS total, o.shipped_at AS shipped_at FROM users u INNER JOIN orders o ON o.user_id = u.id WHERE o.ordered_at > ? AND u.balance >= ?`
"#,
      r#"
import type { DateTime } from "luxon";
import type { Dayjs } from "dayjs";

export type JoinQueryParams = [DateTime, string];

export interface IJoinQueryResult {
	id: number;
	name: UserName;
	shipped_at: Dayjs | null;
	total: string;
}

export interface IJoinQueryQuery {
	params: JoinQueryParams;
	result: IJoinQueryResult;
}
"#,
    )
  }

  #[test]
  fn should_map_param_types_in_subquery() -> Result<(), Box<dyn std::error::Error>> {
    assert_complex_query_types(
      r#"
import { sql } from 'sqlx-ts'
const subqueryQuery = sql`SELECT id, created_at FROM users WHERE id IN (SELECT user_id FROM orders WHERE total > ?)`
"#,
      r#"
import type { DateTime } from "luxon";

export type SubqueryQueryParams = [string];

export interface ISubqueryQueryResult {
	created_at: DateTime;
	id: number;
}

export interface ISubqueryQueryQuery {
	params: SubqueryQueryParams;
	result: ISubqueryQueryResult;
}
"#,
    )
  }

  #[test]
  fn should_map_types_in_cte() -> Result<(), Box<dyn std::error::Error>> {
    assert_complex_query_types(
      r#"
import { sql } from 'sqlx-ts'
const cteQuery = sql`WITH recent AS (SELECT user_id, ordered_at FROM orders WHERE ordered_at > ?) SELECT r.user_id AS user_id, r.ordered_at AS ordered_at FROM recent r`
"#,
      r#"
import type { DateTime } from "luxon";

export type CteQueryParams = [DateTime];

export interface ICteQueryResult {
	ordered_at: DateTime;
	user_id: number;
}

export interface ICteQueryQuery {
	params: CteQueryParams;
	result: ICteQueryResult;
}
"#,
    )
  }

  #[test]
  fn should_map_insert_params_and_returning() -> Result<(), Box<dyn std::error::Error>> {
    assert_complex_query_types(
      r#"
import { sql } from 'sqlx-ts'
const insertQuery = sql`INSERT INTO orders (user_id, total, ordered_at, shipped_at) VALUES (?, ?, ?, ?) RETURNING id, total, ordered_at`
"#,
      r#"
import type { DateTime } from "luxon";
import type { Dayjs } from "dayjs";

export type InsertQueryParams = [[number, string, DateTime, Dayjs | null]];

export interface IInsertQueryResult {
	id: number;
	ordered_at: DateTime;
	total: string;
}

export interface IInsertQueryQuery {
	params: InsertQueryParams;
	result: IInsertQueryResult;
}
"#,
    )
  }

  #[test]
  fn should_map_update_params() -> Result<(), Box<dyn std::error::Error>> {
    assert_complex_query_types(
      r#"
import { sql } from 'sqlx-ts'
const updateQuery = sql`UPDATE users SET balance = ?, deleted_at = ? WHERE created_at < ?`
"#,
      r#"
import type { DateTime } from "luxon";

export type UpdateQueryParams = [string, DateTime | null, DateTime];

export interface IUpdateQueryResult {
	
}

export interface IUpdateQueryQuery {
	params: UpdateQueryParams;
	result: IUpdateQueryResult;
}
"#,
    )
  }

  #[test]
  fn should_map_delete_params() -> Result<(), Box<dyn std::error::Error>> {
    assert_complex_query_types(
      r#"
import { sql } from 'sqlx-ts'
const deleteQuery = sql`DELETE FROM orders WHERE shipped_at < ? AND total < ?`
"#,
      r#"
import type { Dayjs } from "dayjs";

export type DeleteQueryParams = [Dayjs | null, string];

export interface IDeleteQueryResult {
	
}

export interface IDeleteQueryQuery {
	params: DeleteQueryParams;
	result: IDeleteQueryResult;
}
"#,
    )
  }
}
