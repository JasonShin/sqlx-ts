#[cfg(test)]
mod postgres_failure_path_tests {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    /// should be using all CLI args to provide credential for DB connection
    #[test]
    fn failure_with_all_cli_args() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();

        cmd.arg("samples/generic/failure-path1")
            .arg("--db-type=postgres")
            .arg("--db-host=localhost")
            .arg("--db-port=54321")
            .arg("--db-user=postgres")
            .arg("--db-pass=postgres");
        cmd.assert()
            .failure()
            // src/index.ts
            .stderr(predicates::str::contains("relation \"indexjs_unknown\" does not exist"))
            .stderr(predicates::str::contains(
                "INSERT has more expressions than target columns",
            ))
            // src/index.ts -> if statements
            .stderr(predicates::str::contains("relation \"if_statement1\" does not exist"))
            .stderr(predicates::str::contains("relation \"if_statement2\" does not exist"))
            // src/index.ts -> switch statements
            .stderr(predicates::str::contains(
                "relation \"switch_statements1\" does not exist",
            ))
            .stderr(predicates::str::contains(
                "relation \"switch_statements2\" does not exist",
            ))
            // src/index.ts -> for loop statements
            .stderr(predicates::str::contains("relation \"for_loops1\" does not exist"))
            .stderr(predicates::str::contains("relation \"for_loops2\" does not exist"))
            .stderr(predicates::str::contains("relation \"for_loops3\" does not exist"))
            // src/index.ts -> try catch statements
            .stderr(predicates::str::contains("relation \"try1\" does not exist"))
            .stderr(predicates::str::contains("relation \"catch1\" does not exist"))
            .stderr(predicates::str::contains("relation \"throw1\" does not exist"))
            // src/index.ts -> with statement
            .stderr(predicates::str::contains("relation \"with1\" does not exist"))
            // src/index.ts -> while statement
            .stderr(predicates::str::contains("relation \"while1\" does not exist"))
            // src/index.ts -> do while statement
            .stderr(predicates::str::contains("relation \"do_while1\" does not exist"))
            // src/index.ts -> class private method
            .stderr(predicates::str::contains("relation \"class_private1\" does not exist"))
            // src/index.ts -> class public method
            .stderr(predicates::str::contains("relation \"class_public1\" does not exist"))
            // src/index.ts -> class protected method
            .stderr(predicates::str::contains(
                "relation \"class_protected1\" does not exist",
            ))
            // src/import-alias.ts
            .stderr(predicates::str::contains("relation \"aliased_unknown\" does not exist"))
            // src/nested/more-nested/more-nested
            .stderr(predicates::str::contains("relation \"nested_unknown1\" does not exist"))
            .stderr(predicates::str::contains("relation \"nested_unknown2\" does not exist"))
            .stderr(predicates::str::contains("relation \"nested_unknown3\" does not exist"))
            .stdout(predicates::str::contains("SQLs failed to compile!"));

        Ok(())
    }
}
