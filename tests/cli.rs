#[cfg(test)]
mod cli_test {
    use assert_cmd::Command;
    use std::env::current_dir;
    use std::fs;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn all_cli_flags_mysql_with_empty_config() -> Result<(), Box<dyn std::error::Error>> {
        // SETUP
        let demo_path = current_dir().unwrap().join("tests/sample");
        let sample_query_path = demo_path.join("sample.queries.ts");
        let dir = tempdir()?;
        let parent_path = dir.path();
        let config_file_path = parent_path.join(".sqlxrc.json");

        if sample_query_path.exists() { fs::remove_file(&sample_query_path)?; }
        let mut temp_file = fs::File::create(&config_file_path)?;
        let config_content = r#""#;
        writeln!(temp_file, "{}", config_content)?;

        // EXECUTE
        let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();
        cmd.arg(demo_path.to_str().unwrap())
            .arg("--ext=ts")
            .arg(format!("--config={}", config_file_path.to_str().unwrap()))
            .arg("--db-type=mysql")
            .arg("--db-host=127.0.0.1")
            .arg("--db-port=54321")
            .arg("--db-user=postgres")
            .arg("--db-pass=postgres")
            .arg("--db-name=postgres")
            .arg("-g");

        Ok(())
    }

    #[test]
    fn all_cli_flags_postgres_with_empty_config() -> Result<(), Box<dyn std::error::Error>> {
        // SETUP
        let demo_path = current_dir().unwrap().join("tests/sample");
        let sample_query_path = demo_path.join("sample.queries.ts");
        let dir = tempdir()?;
        let parent_path = dir.path();
        let config_file_path = parent_path.join(".sqlxrc.json");

        if sample_query_path.exists() { fs::remove_file(&sample_query_path)?; }
        let mut temp_file = fs::File::create(&config_file_path)?;
        let config_content = r#""#;
        writeln!(temp_file, "{}", config_content)?;

        // EXECUTE
        let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();
        cmd.arg(demo_path.to_str().unwrap())
            .arg("--ext=ts")
            .arg(format!("--config={}", config_file_path.to_str().unwrap()))
            .arg("--db-type=mysql")
            .arg("--db-host=127.0.0.1")
            .arg("--db-port=33306")
            .arg("--db-user=root")
            .arg("--db-pass=")
            .arg("--db-name=sqlx-ts")
            .arg("-g");

        Ok(())
    }

    #[test]
    fn generate_types_work_when_both_cli_and_file_provides_it() -> Result<(), Box<dyn std::error::Error>> {
        // SETUP
        let demo_path = current_dir().unwrap().join("tests/sample");
        let sample_query_path = demo_path.join("sample.queries.ts");
        let dir = tempdir()?;
        let parent_path = dir.path();
        let config_file_path = parent_path.join(".sqlxrc.json");

        if sample_query_path.exists() {
            fs::remove_file(&sample_query_path)?;
        }
        let mut temp_file = fs::File::create(&config_file_path)?;
        let config_content = r#"
{
    "generate_types": {
        "enabled": false
    },
    "connections": {
        "default": {
            "DB_TYPE": "postgres",
            "DB_HOST": "127.0.0.1",
            "DB_PORT": 54321,
            "DB_USER": "postgres",
            "DB_PASS": "postgres",
            "DB_NAME": "postgres"
        }
    }
}"#;
        writeln!(temp_file, "{}", config_content)?;

        // EXECUTE
        let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();
        cmd.arg(demo_path.to_str().unwrap())
            .arg("--ext=ts")
            .arg(format!("--config={}", config_file_path.to_str().unwrap()))
            .arg("-g");

        // ASSERT
        cmd.assert()
            .success()
            .stdout(predicates::str::contains("No SQL errors detected!"));

        assert_eq!(sample_query_path.exists(), true);
        Ok(())
    }

    #[test]
    fn all_cli_flags_should_work_when_file_config_provides_db_configs() -> Result<(), Box<dyn std::error::Error>> {
        // SETUP
        let demo_path = current_dir().unwrap().join("tests/sample");
        let sample_query_path = demo_path.join("sample.queries.ts");
        let dir = tempdir()?;
        let parent_path = dir.path();
        let config_file_path = parent_path.join(".sqlxrc.json");

        if sample_query_path.exists() {
            fs::remove_file(&sample_query_path)?;
        }
        let mut temp_file = fs::File::create(&config_file_path)?;
        let config_content = r#"
{
    "connections": {
        "default": {
            "DB_TYPE": "postgres",
            "DB_HOST": "127.0.0.1",
            "DB_PORT": 54321,
            "DB_USER": "postgres",
            "DB_PASS": "postgres",
            "DB_NAME": "postgres"
        }
    }
}"#;
        writeln!(temp_file, "{}", config_content)?;

        // EXECUTE
        let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();
        cmd.arg(demo_path.to_str().unwrap())
            .arg("--ext=ts")
            .arg(format!("--config={}", config_file_path.to_str().unwrap()))
            .arg("-g");

        // ASSERT
        cmd.assert()
            .success()
            .stdout(predicates::str::contains("No SQL errors detected!"));

        assert_eq!(sample_query_path.exists(), true);
        Ok(())
    }
}
