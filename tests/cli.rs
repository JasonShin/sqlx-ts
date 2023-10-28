#[cfg(test)]
mod cli_test {
    use std::fs;
    use std::env::current_dir;
    use tempfile::tempdir;
    use std::io::Write;
    use assert_cmd::Command;

    #[test]
    fn all_clis_flags_should_work_when_file_config_empty() -> Result<(), Box<dyn std::error::Error>> {
        // SETUP
        let demo_path = current_dir().unwrap().join("tests/demo");
        let dir = tempdir()?;
        let parent_path = dir.path();
        let config_file_path = parent_path.join(".sqlxrc.json");

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
        },
        "db_mysql": {
            "DB_TYPE": "mysql",
            "DB_HOST": "127.0.0.1",
            "DB_PORT": 33306,
            "DB_USER": "root",
            "DB_NAME": "sqlx-ts"
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

        Ok(())
    }
}
