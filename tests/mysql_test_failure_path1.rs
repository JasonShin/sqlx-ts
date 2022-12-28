#[cfg(test)]
mod mysql_failure_path_tests {
    use assert_cmd::prelude::*;
    
    use std::process::Command;

    /// should be using all CLI args to provide credential for DB connection
    #[test]
    fn failure_with_all_cli_args() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();

        cmd.arg("samples/generic/failure-path1")
            .arg("--db-type=mysql")
            .arg("--db-host=127.0.0.1")
            .arg("--db-port=33306")
            .arg("--db-user=root")
            .arg("--db-name=sqlx-ts");
        cmd.assert()
            .failure()
            // src/index.js
            .stderr(predicates::str::contains(
                "Column count doesn\'t match value count at row 1",
            ))
            .stderr(predicates::str::contains(
                "Table \'sqlx-ts.indexjs_unknown\' doesn\'t exist",
            ))
            // src/import-alias.ts
            .stderr(predicates::str::contains(
                "Table \'sqlx-ts.aliased_unknown\' doesn\'t exist",
            ))
            // src/nested/more-nested/more-nested/index.js
            .stderr(predicates::str::contains(
                "Table \'sqlx-ts.nested_unknown1\' doesn\'t exist",
            ))
            .stderr(predicates::str::contains(
                "Table \'sqlx-ts.nested_unknown2\' doesn\'t exist",
            ))
            .stderr(predicates::str::contains(
                "Table \'sqlx-ts.nested_unknown3\' doesn\'t exist",
            ))
            .stdout(predicates::str::contains("SQLs failed to compile!"));

        Ok(())
    }
}
