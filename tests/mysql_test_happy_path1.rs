#[cfg(test)]
mod mysql_happy_path_tests {
    use assert_cmd::prelude::*;
    
    use std::process::Command;

    /// should be using all CLI args to provide credential for DB connection
    #[test]
    fn success_with_all_cli_args() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();

        cmd.arg("samples/generic/happy-path1")
            .arg("--db-type=mysql")
            .arg("--db-host=127.0.0.1")
            .arg("--db-port=33306")
            .arg("--db-name=sqlx-ts")
            .arg("--db-user=root");
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
            .env("DB_PORT", "33306")
            .env("DB_USER", "root")
            .env("DB_NAME", "sqlx-ts");
        cmd.arg("samples/generic/happy-path1").arg("--db-type=mysql");

        cmd.assert()
            .success()
            .stdout(predicates::str::contains("No SQL errors detected!"));

        Ok(())
    }

    #[test]
    fn success_with_partial_env_vars() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();

        cmd.env("DB_HOST", "127.0.0.1")
            .env("DB_PORT", "33306")
            .env("DB_USER", "root")
            .env("DB_NAME", "wrong-db");
        cmd.arg("samples/generic/happy-path1")
            .arg("--db-port=33306")
            .arg("--db-type=mysql")
            .arg("--db-name=sqlx-ts");

        cmd.assert()
            .success()
            .stdout(predicates::str::contains("No SQL errors detected!"));

        Ok(())
    }
}
