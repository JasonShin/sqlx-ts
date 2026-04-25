#[cfg(test)]
mod custom_type_mapping_tests {
  use std::fs;
  use std::io::Write;
  use tempfile::tempdir;

  use assert_cmd::cargo::cargo_bin_cmd;
  use pretty_assertions::assert_eq;
  use test_utils::test_utils::TSString;

  /// Helper: creates a temporary SQLite database, writes a .sqlxrc.json with type_mapping,
  /// runs sqlx-ts, and returns the generated types.
  fn run_type_mapping_test(
    schema_sql: &str,
    ts_content: &str,
    type_mapping_json: &str,
  ) -> Result<(String, String), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let parent_path = dir.path();

    // Create the SQLite database
    let db_path = parent_path.join("test.db");
    let conn = rusqlite::Connection::open(&db_path)?;
    conn.execute_batch(schema_sql)?;
    drop(conn);

    // Write the .sqlxrc.json config with type_mapping
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

    // Write the TS file
    let file_path = parent_path.join("index.ts");
    let mut temp_file = fs::File::create(&file_path)?;
    writeln!(temp_file, "{}", ts_content)?;

    // Run sqlx-ts with CLI args for DB connection + config file for type_mapping
    let mut cmd = cargo_bin_cmd!("sqlx-ts");
    cmd
      .arg(parent_path.to_str().unwrap())
      .arg("--ext=ts")
      .arg("--db-type=sqlite")
      .arg(format!("--db-name={}", db_path.display()))
      .arg(format!("--config={}", config_path.display()))
      .arg("-g");

    let output = cmd.output()?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    assert!(
      output.status.success(),
      "sqlx-ts failed!\nstdout: {stdout}\nstderr: {stderr}"
    );

    // Read generated types
    let type_file_path = parent_path.join("index.queries.ts");
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

    // Should contain the import at the top
    assert!(
      type_file.contains("import type { DateTime } from \"luxon\""),
      "Expected import statement in generated file, got:\n{type_file}"
    );

    // Should use the custom type
    assert!(
      type_file.contains("created_at: DateTime;"),
      "Expected DateTime type for created_at, got:\n{type_file}"
    );

    Ok(())
  }

  #[test]
  fn should_not_override_unmapped_types() -> Result<(), Box<dyn std::error::Error>> {
    let schema = "CREATE TABLE test_custom_types (id INTEGER PRIMARY KEY NOT NULL, name TEXT NOT NULL, count BIGINT NOT NULL);";

    let ts_content = r#"
import { sql } from 'sqlx-ts'
const someQuery = sql`SELECT * FROM test_custom_types`
"#;

    // Only override bigint, text should remain string
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
}
