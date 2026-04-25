#[cfg(test)]
mod sqlite_query_parameters_tests {
  use std::env;
  use std::fs;
  use std::io::Write;
  use tempfile::tempdir;

  use assert_cmd::cargo::cargo_bin_cmd;
  use pretty_assertions::assert_eq;
  use test_utils::test_utils::TSString;

  /// Helper: creates a temporary SQLite database with the given schema,
  /// then runs sqlx-ts on the given TS content, and returns the generated types.
  fn run_sqlite_test(
    schema_sql: &str,
    ts_content: &str,
    generate_types: bool,
  ) -> Result<(String, String), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let parent_path = dir.path();

    // Create the SQLite database and populate it with the schema
    let db_path = parent_path.join("test.db");
    let conn = rusqlite::Connection::open(&db_path)?;
    conn.execute_batch(schema_sql)?;
    drop(conn);

    // Write the TS file
    let file_path = parent_path.join("index.ts");
    let mut temp_file = fs::File::create(&file_path)?;
    writeln!(temp_file, "{}", ts_content)?;

    // Run sqlx-ts
    let mut cmd = cargo_bin_cmd!("sqlx-ts");
    cmd
      .arg(parent_path.to_str().unwrap())
      .arg("--ext=ts")
      .arg("--db-type=sqlite")
      .arg(format!("--db-name={}", db_path.display()));

    if generate_types {
      cmd.arg("-g");
    }

    let output = cmd.output()?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    assert!(
      output.status.success(),
      "sqlx-ts failed!\nstdout: {stdout}\nstderr: {stderr}"
    );
    assert!(
      stdout.contains("No SQL errors detected!"),
      "Expected success message in stdout: {stdout}"
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
  fn should_validate_simple_select() -> Result<(), Box<dyn std::error::Error>> {
    let schema = "CREATE TABLE items (id INTEGER PRIMARY KEY NOT NULL, name TEXT NOT NULL, price REAL);";

    let ts_content = r#"
import { sql } from 'sqlx-ts'

const someQuery = sql`SELECT * FROM items`
"#;

    let (_, type_file) = run_sqlite_test(schema, ts_content, true)?;

    let expected = r#"
export type SomeQueryParams = [];

export interface ISomeQueryResult {
	id: number;
	name: string;
	price: number | null;
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
  fn should_handle_query_params_with_question_mark() -> Result<(), Box<dyn std::error::Error>> {
    let schema = "CREATE TABLE items (id INTEGER PRIMARY KEY NOT NULL, name TEXT NOT NULL, price REAL);";

    let ts_content = r#"
import { sql } from 'sqlx-ts'

const someQuery = sql`SELECT * FROM items WHERE id = ? AND name = ?`
"#;

    let (_, type_file) = run_sqlite_test(schema, ts_content, true)?;

    let expected = r#"
export type SomeQueryParams = [number, string];

export interface ISomeQueryResult {
	id: number;
	name: string;
	price: number | null;
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
  fn should_handle_insert_with_params() -> Result<(), Box<dyn std::error::Error>> {
    let schema = "CREATE TABLE items (id INTEGER PRIMARY KEY NOT NULL, name TEXT NOT NULL, price REAL);";

    let ts_content = r#"
import { sql } from 'sqlx-ts'

const someQuery = sql`INSERT INTO items (name, price) VALUES (?, ?)`
"#;

    let (_, type_file) = run_sqlite_test(schema, ts_content, true)?;

    let expected = r#"
export type SomeQueryParams = [[string, number | null]];

export interface ISomeQueryResult {
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
  fn should_handle_multiple_types() -> Result<(), Box<dyn std::error::Error>> {
    let schema = r"
      CREATE TABLE events (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT NOT NULL,
        description TEXT,
        start_date DATETIME,
        is_active BOOLEAN NOT NULL DEFAULT 1,
        score REAL,
        metadata JSON
      );
    ";

    let ts_content = r#"
import { sql } from 'sqlx-ts'

const someQuery = sql`SELECT * FROM events WHERE id = ?`
"#;

    let (_, type_file) = run_sqlite_test(schema, ts_content, true)?;

    let expected = r#"
export type SomeQueryParams = [number];

export interface ISomeQueryResult {
	description: string | null;
	id: number;
	is_active: boolean;
	metadata: object | null;
	name: string;
	score: number | null;
	start_date: Date | null;
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
  fn should_detect_invalid_sql() -> Result<(), Box<dyn std::error::Error>> {
    let schema = "CREATE TABLE items (id INTEGER PRIMARY KEY NOT NULL, name TEXT NOT NULL);";

    let ts_content = r#"
import { sql } from 'sqlx-ts'

const someQuery = sql`SELECT * FROM nonexistent_table`
"#;

    let dir = tempdir()?;
    let parent_path = dir.path();

    let db_path = parent_path.join("test.db");
    let conn = rusqlite::Connection::open(&db_path)?;
    conn.execute_batch(schema)?;
    drop(conn);

    let file_path = parent_path.join("index.ts");
    let mut temp_file = fs::File::create(&file_path)?;
    writeln!(temp_file, "{}", ts_content)?;

    let mut cmd = cargo_bin_cmd!("sqlx-ts");
    cmd
      .arg(parent_path.to_str().unwrap())
      .arg("--ext=ts")
      .arg("--db-type=sqlite")
      .arg(format!("--db-name={}", db_path.display()));

    // This should fail because the table doesn't exist
    let output = cmd.output()?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    // The command should report SQL errors
    assert!(
      !stdout.contains("No SQL errors detected!"),
      "Expected SQL errors but got success: {stdout}"
    );
    Ok(())
  }
}
