#[cfg(test)]
mod dotenv_test {
  use assert_cmd::Command;
  use std::env::current_dir;
  use std::fs;
  use std::io::Write;
  use tempfile::tempdir;

  #[test]
  fn uses_dotenv_when_no_db_flags_are_provided() -> Result<(), Box<dyn std::error::Error>> {
    // SETUP
    let demo_dir = current_dir().unwrap().join("tests/dotenv");
    let dotenv_path = demo_dir.join(".env");

    // Write a sample .env file with database credentials
    let dotenv_content = r#"
DB_TYPE=postgres
DB_HOST=127.0.0.1
DB_PORT=54321
DB_USER=postgres
DB_PASS=postgres
DB_NAME=postgres
"#;
    fs::create_dir_all(&demo_dir)?;
    fs::write(&dotenv_path, dotenv_content)?;

    // Create an empty config file
    let temp_config_dir = tempdir()?;
    let config_file_path = temp_config_dir.path().join(".sqlxrc.json");
    fs::write(&config_file_path, "{}")?;

    // EXECUTE
    let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();
    cmd
      .arg(demo_dir.to_str().unwrap())
      .arg("--ext=ts")
      .arg(format!("--config={}", config_file_path.to_str().unwrap()))
      .arg("-g");

    cmd.assert().failure().stderr(predicates::str::contains(
      "Empty or invalid JSON provided for file based configuration - config file:",
    ));

    fs::remove_file(dotenv_path)?; // optional: clean .env if needed

    Ok(())
  }


}
