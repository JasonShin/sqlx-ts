#[cfg(test)]
mod exit_code_tests {
  use assert_cmd::prelude::*;
  use std::fs;
  use std::io::Write;
  use std::process::Command;
  use tempfile::tempdir;

  /// This test verifies that even when there are multiple SQL queries with mixed
  /// success/failure, the tool still exits with code 1
  #[test]
  fn should_exit_with_code_1_when_sql_errors_detected() -> Result<(), Box<dyn std::error::Error>> {
    // SETUP
    let dir = tempdir()?;
    let parent_path = dir.path();
    let file_path = parent_path.join("index.ts");

    let index_content = r#"
import { sql } from "sqlx-ts";

// This should succeed
const validQuery = sql`SELECT id, name FROM characters WHERE id = $1;`;

// This should fail - unknown table
const invalidQuery = sql`SELECT * FROM unknown_table;`;

// Another valid query after the failure
const anotherValidQuery = sql`SELECT * FROM inventory WHERE character_id = $1;`;
    "#;
    let mut temp_file = fs::File::create(&file_path)?;
    writeln!(temp_file, "{}", index_content)?;

    // EXECUTE
    let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();

    cmd
      .arg(parent_path.to_str().unwrap())
      .arg("--ext=ts")
      .arg("--db-type=postgres")
      .arg("--db-host=127.0.0.1")
      .arg("--db-port=54321")
      .arg("--db-user=postgres")
      .arg("--db-pass=postgres")
      .arg("--db-name=postgres");

    // ASSERT - should exit with non-zero code due to the error
    cmd
      .assert()
      .failure()
      .stderr(predicates::str::contains("relation \"unknown_table\" does not exist"))
      .stderr(predicates::str::contains("SQLs failed to compile!"));

    Ok(())
  }

  /// Test that when all queries succeed, exit code is 0
  #[test]
  fn should_exit_with_code_0_when_no_errors() -> Result<(), Box<dyn std::error::Error>> {
    // SETUP
    let dir = tempdir()?;
    let parent_path = dir.path();
    let file_path = parent_path.join("valid.ts");

    let index_content = r#"
import { sql } from "sqlx-ts";

const query1 = sql`SELECT id, name FROM characters WHERE id = $1;`;
const query2 = sql`SELECT * FROM inventory WHERE character_id = $1;`;
const query3 = sql`SELECT * FROM items WHERE id = $1;`;
    "#;
    let mut temp_file = fs::File::create(&file_path)?;
    writeln!(temp_file, "{}", index_content)?;

    // EXECUTE
    let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();

    cmd
      .arg(parent_path.to_str().unwrap())
      .arg("--ext=ts")
      .arg("--db-type=postgres")
      .arg("--db-host=127.0.0.1")
      .arg("--db-port=54321")
      .arg("--db-user=postgres")
      .arg("--db-pass=postgres")
      .arg("--db-name=postgres");

    // ASSERT - should succeed
    cmd
      .assert()
      .success()
      .stdout(predicates::str::contains("No SQL errors detected!"));

    Ok(())
  }

  /// Test with many successes and one failure in the middle
  /// Pattern: 5 successes -> 1 failure -> 5 successes
  /// Expected: exit code 1
  #[test]
  fn should_fail_with_many_successes_and_one_failure_in_middle() -> Result<(), Box<dyn std::error::Error>> {
    // SETUP
    let dir = tempdir()?;
    let parent_path = dir.path();
    let file_path = parent_path.join("one_failure.ts");

    let index_content = r#"
import { sql } from "sqlx-ts";

const query1 = sql`SELECT id FROM characters WHERE id = $1;`;
const query2 = sql`SELECT name FROM characters WHERE id = $1;`;
const query3 = sql`SELECT * FROM inventory WHERE id = $1;`;
const query4 = sql`SELECT * FROM items WHERE id = $1;`;
const query5 = sql`SELECT quantity FROM inventory WHERE id = $1;`;

// Single failure in the middle
const failedQuery = sql`SELECT * FROM this_table_does_not_exist;`;

const query6 = sql`SELECT rarity FROM items WHERE id = $1;`;
const query7 = sql`SELECT character_id FROM inventory WHERE id = $1;`;
const query8 = sql`SELECT flavor_text FROM items WHERE id = $1;`;
const query9 = sql`SELECT id, quantity FROM inventory WHERE character_id = $1;`;
const query10 = sql`SELECT id, name FROM characters LIMIT 10;`;
    "#;
    let mut temp_file = fs::File::create(&file_path)?;
    writeln!(temp_file, "{}", index_content)?;

    // EXECUTE
    let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();

    cmd
      .arg(parent_path.to_str().unwrap())
      .arg("--ext=ts")
      .arg("--db-type=postgres")
      .arg("--db-host=127.0.0.1")
      .arg("--db-port=54321")
      .arg("--db-user=postgres")
      .arg("--db-pass=postgres")
      .arg("--db-name=postgres");

    // ASSERT - should fail despite 10 successes and only 1 failure
    cmd
      .assert()
      .failure()
      .stderr(predicates::str::contains(
        "relation \"this_table_does_not_exist\" does not exist",
      ))
      .stderr(predicates::str::contains("SQLs failed to compile!"));

    Ok(())
  }

  /// Test with multiple files: one success file and one failure file
  /// Expected: exit code 1
  #[test]
  fn should_fail_with_multiple_files_one_success_one_failure() -> Result<(), Box<dyn std::error::Error>> {
    // SETUP
    let dir = tempdir()?;
    let parent_path = dir.path();

    // File 1: All successful queries
    let file1_path = parent_path.join("success.ts");
    let file1_content = r#"
import { sql } from "sqlx-ts";

const query1 = sql`SELECT id, name FROM characters WHERE id = $1;`;
const query2 = sql`SELECT * FROM inventory WHERE character_id = $1;`;
const query3 = sql`SELECT * FROM items WHERE id = $1;`;
    "#;
    let mut file1 = fs::File::create(&file1_path)?;
    writeln!(file1, "{}", file1_content)?;

    // File 2: Contains failures
    let file2_path = parent_path.join("failure.ts");
    let file2_content = r#"
import { sql } from "sqlx-ts";

const failQuery1 = sql`SELECT * FROM nonexistent_table;`;
const failQuery2 = sql`SELECT * FROM another_missing_table;`;
    "#;
    let mut file2 = fs::File::create(&file2_path)?;
    writeln!(file2, "{}", file2_content)?;

    // EXECUTE
    let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();

    cmd
      .arg(parent_path.to_str().unwrap())
      .arg("--ext=ts")
      .arg("--db-type=postgres")
      .arg("--db-host=127.0.0.1")
      .arg("--db-port=54321")
      .arg("--db-user=postgres")
      .arg("--db-pass=postgres")
      .arg("--db-name=postgres");

    // ASSERT - should fail because file2 has errors
    cmd
      .assert()
      .failure()
      .stderr(predicates::str::contains(
        "relation \"nonexistent_table\" does not exist",
      ))
      .stderr(predicates::str::contains(
        "relation \"another_missing_table\" does not exist",
      ))
      .stderr(predicates::str::contains("SQLs failed to compile!"));

    Ok(())
  }

  /// Test with multiple files: all files contain successful queries
  /// Expected: exit code 0
  #[test]
  fn should_succeed_with_multiple_files_all_successful() -> Result<(), Box<dyn std::error::Error>> {
    // SETUP
    let dir = tempdir()?;
    let parent_path = dir.path();

    // File 1: Successful queries
    let file1_path = parent_path.join("queries1.ts");
    let file1_content = r#"
import { sql } from "sqlx-ts";

const query1 = sql`SELECT id FROM characters WHERE id = $1;`;
const query2 = sql`SELECT name FROM characters WHERE id = $1;`;
    "#;
    let mut file1 = fs::File::create(&file1_path)?;
    writeln!(file1, "{}", file1_content)?;

    // File 2: More successful queries
    let file2_path = parent_path.join("queries2.ts");
    let file2_content = r#"
import { sql } from "sqlx-ts";

const query3 = sql`SELECT * FROM inventory WHERE id = $1;`;
const query4 = sql`SELECT * FROM items WHERE id = $1;`;
    "#;
    let mut file2 = fs::File::create(&file2_path)?;
    writeln!(file2, "{}", file2_content)?;

    // File 3: Even more successful queries
    let file3_path = parent_path.join("queries3.ts");
    let file3_content = r#"
import { sql } from "sqlx-ts";

const query5 = sql`SELECT quantity FROM inventory WHERE character_id = $1;`;
const query6 = sql`SELECT rarity FROM items WHERE id = $1;`;
    "#;
    let mut file3 = fs::File::create(&file3_path)?;
    writeln!(file3, "{}", file3_content)?;

    // EXECUTE
    let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();

    cmd
      .arg(parent_path.to_str().unwrap())
      .arg("--ext=ts")
      .arg("--db-type=postgres")
      .arg("--db-host=127.0.0.1")
      .arg("--db-port=54321")
      .arg("--db-user=postgres")
      .arg("--db-pass=postgres")
      .arg("--db-name=postgres");

    // ASSERT - should succeed
    cmd
      .assert()
      .success()
      .stdout(predicates::str::contains("No SQL errors detected!"));

    Ok(())
  }
}
