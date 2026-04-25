#[cfg(test)]
mod demo_happy_path_tests {
  use assert_cmd::cargo::cargo_bin_cmd;
  use pretty_assertions::assert_eq;
  use std::env;
  use std::env::current_dir;
  use std::fs;
  use std::io::Write;
  use std::path::Path;
  use tempfile::tempdir;
  use walkdir::WalkDir;

  fn run_demo_test(demo_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // EXECUTE - Generate types for .ts files
    let mut cmd = cargo_bin_cmd!("sqlx-ts");
    cmd
      .arg(demo_path.to_str().unwrap())
      .arg("--ext=ts")
      .arg("--config=.sqlxrc.sample.json")
      .arg("-g");

    // ASSERT
    cmd
      .assert()
      .success()
      .stdout(predicates::str::contains("No SQL errors detected!"));

    // Also generate types for other extensions in file_extensions directory
    let file_extensions_path = demo_path.join("file_extensions");
    if file_extensions_path.exists() {
      for ext in &["js", "mts", "cts", "mjs", "cjs"] {
        let mut cmd = cargo_bin_cmd!("sqlx-ts");
        cmd
          .arg(file_extensions_path.to_str().unwrap())
          .arg(format!("--ext={}", ext))
          .arg("--config=.sqlxrc.sample.json")
          .arg("-g");
        cmd
          .assert()
          .success()
          .stdout(predicates::str::contains("No SQL errors detected!"));
      }
    }

    // Also generate types for SQL files
    let sql_files_path = demo_path.join("sql_files");
    if sql_files_path.exists() {
      let mut cmd = cargo_bin_cmd!("sqlx-ts");
      cmd
        .arg(sql_files_path.to_str().unwrap())
        .arg("--ext=sql")
        .arg("--config=.sqlxrc.sample.json")
        .arg("-g");
      cmd
        .assert()
        .success()
        .stdout(predicates::str::contains("No SQL errors detected!"));
    }

    // Verify all generated types match snapshots
    for entry in WalkDir::new(demo_path) {
      if entry.is_ok() {
        let entry = entry.unwrap();
        let path = entry.path();
        let parent = entry.path().parent().unwrap();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();

        if path.is_file() && file_name.ends_with(".queries.ts") {
          let base_file_name = file_name.split('.').collect::<Vec<&str>>();
          let base_file_name = base_file_name.first().unwrap();
          let snapshot_path = parent.join(format!("{base_file_name}.snapshot.ts"));

          let generated_types = fs::read_to_string(path)?;

          if !snapshot_path.exists() {
            let mut snapshot_file = fs::File::create(&snapshot_path)?;
            writeln!(snapshot_file, "{generated_types}")?;
          }

          assert_eq!(
            generated_types.trim().to_string().trim(),
            fs::read_to_string(&snapshot_path)?.to_string().trim(),
          )
        }
      }
    }

    Ok(())
  }

  #[test]
  fn all_demo_should_pass() -> Result<(), Box<dyn std::error::Error>> {
    let root_path = current_dir().unwrap();
    let demo_path = root_path.join("tests/demo");
    run_demo_test(&demo_path)
  }

  #[test]
  fn all_demo_json_postgres() -> Result<(), Box<dyn std::error::Error>> {
    // PostgreSQL JSON tests - compatible with all PostgreSQL versions that support JSON
    let root_path = current_dir().unwrap();
    let demo_path = root_path.join("tests/demo_json/postgres");
    run_demo_test(&demo_path)
  }

  #[test]
  fn all_demo_json_mysql() -> Result<(), Box<dyn std::error::Error>> {
    // MySQL 5.7+ and PostgreSQL JSON tests
    if env::var("MYSQL_VERSION").ok() == Some("5.6".to_string()) {
      return Ok(()); // Skip test for MySQL 5.6 which doesn't support JSON functions
    }

    let root_path = current_dir().unwrap();
    let demo_path = root_path.join("tests/demo_json/mysql");
    run_demo_test(&demo_path)
  }

  #[test]
  fn test_js_files() -> Result<(), Box<dyn std::error::Error>> {
    // SETUP
    let root_path = current_dir().unwrap();
    let demo_path = root_path.join("tests/demo/file_extensions");

    // EXECUTE
    let mut cmd = cargo_bin_cmd!("sqlx-ts");
    cmd
      .arg(demo_path.to_str().unwrap())
      .arg("--ext=js")
      .arg("--config=.sqlxrc.sample.json")
      .arg("-g");

    // ASSERT
    cmd
      .assert()
      .success()
      .stdout(predicates::str::contains("Found 2 SQL queries"))
      .stdout(predicates::str::contains("No SQL errors detected!"));

    Ok(())
  }

  #[test]
  fn test_mts_files() -> Result<(), Box<dyn std::error::Error>> {
    // SETUP
    let root_path = current_dir().unwrap();
    let demo_path = root_path.join("tests/demo/file_extensions");

    // EXECUTE
    let mut cmd = cargo_bin_cmd!("sqlx-ts");
    cmd
      .arg(demo_path.to_str().unwrap())
      .arg("--ext=mts")
      .arg("--config=.sqlxrc.sample.json")
      .arg("-g");

    // ASSERT
    cmd
      .assert()
      .success()
      .stdout(predicates::str::contains("Found 2 SQL queries"))
      .stdout(predicates::str::contains("No SQL errors detected!"));

    Ok(())
  }

  #[test]
  fn test_cts_files() -> Result<(), Box<dyn std::error::Error>> {
    // SETUP
    let root_path = current_dir().unwrap();
    let demo_path = root_path.join("tests/demo/file_extensions");

    // EXECUTE
    let mut cmd = cargo_bin_cmd!("sqlx-ts");
    cmd
      .arg(demo_path.to_str().unwrap())
      .arg("--ext=cts")
      .arg("--config=.sqlxrc.sample.json")
      .arg("-g");

    // ASSERT
    cmd
      .assert()
      .success()
      .stdout(predicates::str::contains("Found 2 SQL queries"))
      .stdout(predicates::str::contains("No SQL errors detected!"));

    Ok(())
  }

  #[test]
  fn test_mjs_files() -> Result<(), Box<dyn std::error::Error>> {
    // SETUP
    let root_path = current_dir().unwrap();
    let demo_path = root_path.join("tests/demo/file_extensions");

    // EXECUTE
    let mut cmd = cargo_bin_cmd!("sqlx-ts");
    cmd
      .arg(demo_path.to_str().unwrap())
      .arg("--ext=mjs")
      .arg("--config=.sqlxrc.sample.json")
      .arg("-g");

    // ASSERT
    cmd
      .assert()
      .success()
      .stdout(predicates::str::contains("Found 2 SQL queries"))
      .stdout(predicates::str::contains("No SQL errors detected!"));

    Ok(())
  }

  #[test]
  fn test_cjs_files() -> Result<(), Box<dyn std::error::Error>> {
    // SETUP
    let root_path = current_dir().unwrap();
    let demo_path = root_path.join("tests/demo/file_extensions");

    // EXECUTE
    let mut cmd = cargo_bin_cmd!("sqlx-ts");
    cmd
      .arg(demo_path.to_str().unwrap())
      .arg("--ext=cjs")
      .arg("--config=.sqlxrc.sample.json")
      .arg("-g");

    // ASSERT
    cmd
      .assert()
      .success()
      .stdout(predicates::str::contains("Found 2 SQL queries"))
      .stdout(predicates::str::contains("No SQL errors detected!"));

    Ok(())
  }

  #[test]
  fn test_sql_files() -> Result<(), Box<dyn std::error::Error>> {
    // SETUP
    let root_path = current_dir().unwrap();
    let demo_path = root_path.join("tests/demo/sql_files");

    // EXECUTE
    let mut cmd = cargo_bin_cmd!("sqlx-ts");
    cmd
      .arg(demo_path.to_str().unwrap())
      .arg("--ext=sql")
      .arg("--config=.sqlxrc.sample.json")
      .arg("-g");

    // ASSERT
    cmd
      .assert()
      .success()
      .stdout(predicates::str::contains("Found 5 SQL queries"))
      .stdout(predicates::str::contains("No SQL errors detected!"));

    Ok(())
  }

  #[test]
  fn test_multiple_extensions() -> Result<(), Box<dyn std::error::Error>> {
    // SETUP
    let root_path = current_dir().unwrap();
    let demo_path = root_path.join("tests/demo/file_extensions");

    // EXECUTE - Test scanning multiple extensions at once
    let mut cmd = cargo_bin_cmd!("sqlx-ts");
    cmd
      .arg(demo_path.to_str().unwrap())
      .arg("--ext=js")
      .arg("--ext=mts")
      .arg("--ext=cjs")
      .arg("--config=.sqlxrc.sample.json")
      .arg("-g");

    // ASSERT
    cmd
      .assert()
      .success()
      .stdout(predicates::str::contains("Found 6 SQL queries"))
      .stdout(predicates::str::contains("No SQL errors detected!"));

    Ok(())
  }

  #[test]
  fn all_demo_sqlite_should_pass() -> Result<(), Box<dyn std::error::Error>> {
    let root_path = current_dir().unwrap();
    let demo_path = root_path.join("tests/demo_sqlite");
    let migration_path = root_path.join("playpen/db/sqlite_migration.sql");

    // Create a temporary SQLite database and run the migration
    let tmp_dir = tempdir()?;
    let db_path = tmp_dir.path().join("demo_test.db");
    let conn = rusqlite::Connection::open(&db_path)?;
    let migration_sql = fs::read_to_string(&migration_path)?;
    conn.execute_batch(&migration_sql)?;
    drop(conn);

    // Create a temporary config file pointing to the SQLite database
    let config_path = tmp_dir.path().join(".sqlxrc.json");
    let config_content = format!(
      r#"{{
  "generateTypes": {{
    "enabled": true
  }},
  "connections": {{
    "default": {{
      "DB_TYPE": "sqlite",
      "DB_NAME": "{}"
    }}
  }}
}}"#,
      db_path.display()
    );
    fs::write(&config_path, &config_content)?;

    // Run sqlx-ts against the demo_sqlite directory
    // Use --db-type and --db-name CLI args to override any .env file values
    let mut cmd = cargo_bin_cmd!("sqlx-ts");
    cmd
      .arg(demo_path.to_str().unwrap())
      .arg("--ext=ts")
      .arg(format!("--config={}", config_path.display()))
      .arg("--db-type=sqlite")
      .arg(format!("--db-name={}", db_path.display()))
      .arg("-g");

    cmd
      .assert()
      .success()
      .stdout(predicates::str::contains("No SQL errors detected!"));

    // Verify all generated types match snapshots
    for entry in WalkDir::new(&demo_path) {
      if entry.is_ok() {
        let entry = entry.unwrap();
        let path = entry.path();
        let parent = entry.path().parent().unwrap();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();

        if path.is_file() && file_name.ends_with(".queries.ts") {
          let base_file_name = file_name.split('.').collect::<Vec<&str>>();
          let base_file_name = base_file_name.first().unwrap();
          let snapshot_path = parent.join(format!("{base_file_name}.snapshot.ts"));

          let generated_types = fs::read_to_string(path)?;

          if !snapshot_path.exists() {
            let mut snapshot_file = fs::File::create(&snapshot_path)?;
            writeln!(snapshot_file, "{generated_types}")?;
          }

          assert_eq!(
            generated_types.trim().to_string().trim(),
            fs::read_to_string(&snapshot_path)?.to_string().trim(),
          )
        }
      }
    }

    Ok(())
  }
}
