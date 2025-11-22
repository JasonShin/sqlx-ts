#[cfg(test)]
mod demo_happy_path_tests {
  use assert_cmd::prelude::*;
  use pretty_assertions::assert_eq;
  use std::env::current_dir;
  use std::fs;
  use std::io::Write;
  use std::process::Command;
  use walkdir::WalkDir;

  #[test]
  fn all_demo_should_pass() -> Result<(), Box<dyn std::error::Error>> {
    // SETUP
    let root_path = current_dir().unwrap();
    let demo_path = root_path.join("tests/demo");

    // EXECUTE
    let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();
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
  fn test_js_files() -> Result<(), Box<dyn std::error::Error>> {
    // SETUP
    let root_path = current_dir().unwrap();
    let demo_path = root_path.join("tests/demo/file_extensions");

    // EXECUTE
    let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();
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
    let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();
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
    let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();
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
    let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();
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
    let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();
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
    let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();
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
    let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();
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
}
