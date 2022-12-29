#[cfg(test)]
mod postgres_test_happy_path_tests {
    use assert_cmd::prelude::*;

    use std::process::Command;

    /// should be using all CLI args to provide credential for DB connection
    #[test]
    fn success_with_all_cli_args() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();

        cmd.arg("samples/generic/js-happy-path1")
            .arg("--ext=js")
            .arg("--db-type=postgres")
            .arg("--db-host=localhost")
            .arg("--db-port=54321")
            .arg("--db-user=postgres")
            .arg("--db-pass=postgres");
        cmd.assert()
            .success()
            .stdout(predicates::str::contains("No SQL errors detected!"));

        Ok(())
    }

    /// should not be using any arg to provide credential for DB connection
    #[test]
    fn success_with_env_vars() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();

        cmd.env("DB_HOST", "127.0.0.1")
            .env("DB_PORT", "54321")
            .env("DB_USER", "postgres");
        cmd.arg("samples/generic/js-happy-path1")
            .arg("--ext=js")
            .arg("--db-type=postgres")
            .arg("--db-pass=postgres");

        cmd.assert()
            .success()
            .stdout(predicates::str::contains("No SQL errors detected!"));

        Ok(())
    }

    #[test]
    fn success_with_partial_env_vars() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();

        cmd.env("DB_HOST", "127.0.0.1")
            .env("DB_TYPE", "postgres")
            .env("DB_PORT", "33306")
            .env("DB_USER", "postgres");

        cmd.arg("samples/generic/js-happy-path1")
            .arg("--ext=js")
            .arg("--db-port=54321")
            .arg("--db-pass=postgres");

        cmd.assert()
            .success()
            .stdout(predicates::str::contains("No SQL errors detected!"));

        Ok(())
    }
}
