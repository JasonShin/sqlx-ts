#[cfg(test)]
mod dotenv_test {
  use assert_cmd::Command;
  use std::fs::{self};
  
  use tempfile::tempdir;

  #[test]
  fn loads_env_vars_from_dotenv_file() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let dotenv_path = temp_dir.path().join(".env");

    let dotenv_content = r#"
DB_TYPE=postgres
DB_HOST=127.0.0.1
DB_PORT=54321
DB_USER=postgres
DB_PASS=postgres
DB_NAME=postgres
"#;
    fs::write(&dotenv_path, dotenv_content)?;

    let sample_dir = temp_dir.path().join("sample");
    fs::create_dir_all(&sample_dir)?;
    fs::copy("tests/sample/sample.ts", sample_dir.join("sample.ts"))?;

    let mut cmd = Command::cargo_bin("sqlx-ts")?;
    cmd
      .current_dir(temp_dir.path())
      .arg("--ext=ts")
      .arg("-g")
      .arg(sample_dir.to_str().unwrap());

    cmd
      .assert()
      .success()
      .stdout(predicates::str::contains("No SQL errors detected!")); // adjust if your CLI errors differently

    Ok(())
  }

  #[test]
  fn loads_env_vars_from_dotenv_file_but_wrong_config() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let dotenv_path = temp_dir.path().join(".env");

    let dotenv_content = r#"
DB_TYPE=postgres
DB_HOST=127.0.0.1
DB_PORT=54322
DB_USER=postgres
DB_PASS=postgres
DB_NAME=postgres
"#;
    fs::write(&dotenv_path, dotenv_content)?;

    let sample_dir = temp_dir.path().join("sample");
    fs::create_dir_all(&sample_dir)?;
    fs::copy("tests/sample/sample.ts", sample_dir.join("sample.ts"))?;

    let mut cmd = Command::cargo_bin("sqlx-ts")?;
    cmd
      .current_dir(temp_dir.path())
      .arg("--ext=ts")
      .arg("-g")
      .arg(sample_dir.to_str().unwrap());

    cmd.assert().failure().stderr(predicates::str::contains(
      "Postgres database connection error: error connecting to server: Connection refused",
    )); // adjust if your CLI errors differently

    Ok(())
  }

  #[test]
  fn fails_if_dotenv_missing() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;

    let sample_dir = temp_dir.path().join("sample");
    fs::create_dir_all(&sample_dir)?;
    fs::copy("tests/sample/sample.ts", sample_dir.join("sample.ts"))?;

    let mut cmd = Command::cargo_bin("sqlx-ts")?;
    cmd
      .current_dir(temp_dir.path())
      .arg("--ext=ts")
      .arg("-g")
      .arg(sample_dir.to_str().unwrap());

    cmd.assert().failure().stderr(predicates::str::contains(
      "Unable to retrieve a database type, please check your configuration and try again",
    ));

    Ok(())
  }
}
