#[cfg(test)]
mod js_mysql_failure_path_tests {
    use std::fs;
    use assert_cmd::prelude::*;
    use std::process::Command;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn fails_insert_column_count_mismatch() -> Result<(), Box<dyn std::error::Error>> {
        // SETUP
        let dir = tempdir()?;
        let parent_path = dir.path();
        let file_path = parent_path.join("index.ts");

        let index_content = r#"
import { sql } from "sqlx-ts";

const insertQuery = sql`
INSERT INTO items (food_type, time_takes_to_cook, table_id, points)
VALUES ('steak', 1, 1, 1, 1);
`;
        "#;
        let mut temp_file = fs::File::create(&file_path)?;
        writeln!(temp_file, "{}", index_content)?;

        // EXECUTE
        let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();

        cmd.arg(parent_path.to_str().unwrap())
            .arg("--ext=ts")
            .arg("--db-type=mysql")
            .arg("--db-host=127.0.0.1")
            .arg("--db-port=33306")
            .arg("--db-user=root")
            .arg("--db-name=sqlx-ts")
            .arg("-g");

        // ASSERT
        cmd.assert()
            .failure()
            .stderr(predicates::str::contains(
                "Column count doesn't match value count at row 1",
            ));
        Ok(())
    }

    #[test]
    fn fails_to_find_an_unknown_table() -> Result<(), Box<dyn std::error::Error>> {
        // SETUP
        let dir = tempdir()?;
        let parent_path = dir.path();
        let file_path = parent_path.join("index.ts");

        let index_content = r#"
import { sql } from "sqlx-ts";

const someQuery = sql`SELECT * FROM index_unknown`;
        "#;
        let mut temp_file = fs::File::create(&file_path)?;
        writeln!(temp_file, "{}", index_content)?;

        // EXECUTE
        let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();

        cmd.arg(parent_path.to_str().unwrap())
            .arg("--ext=ts")
            .arg("--db-type=mysql")
            .arg("--db-host=127.0.0.1")
            .arg("--db-port=33306")
            .arg("--db-user=root")
            .arg("--db-name=sqlx-ts")
            .arg("-g");

        // ASSERT
        cmd.assert()
            .failure()
            .stderr(predicates::str::contains(
                "Table \'sqlx-ts.index_unknown\' doesn\'t exist",
            ));
        Ok(())
    }


    #[test]
    fn fails_to_find_an_unknown_table_nested() -> Result<(), Box<dyn std::error::Error>> {
        // SETUP
        let dir = tempdir()?;
        let parent_path = dir.path();
        let file_path = parent_path.join("nested_1/nested_2/index.ts");

        let index_content = r#"
import { sql } from "sqlx-ts";

const someQuery = sql`SELECT * FROM nested_unknown`;
        "#;
        fs::create_dir_all(file_path.parent().unwrap())?;
        let mut temp_file = fs::File::create(&file_path)?;
        writeln!(temp_file, "{}", index_content)?;

        // EXECUTE
        let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();

        cmd.arg(parent_path.to_str().unwrap())
            .arg("--ext=ts")
            .arg("--db-type=mysql")
            .arg("--db-host=127.0.0.1")
            .arg("--db-port=33306")
            .arg("--db-user=root")
            .arg("--db-name=sqlx-ts")
            .arg("-g");

        // ASSERT
        cmd.assert()
            .failure()
            .stderr(predicates::str::contains(
                "Table \'sqlx-ts.nested_unknown\' doesn\'t exist",
            ));
        Ok(())
    }

    #[test]
    fn fails_to_find_an_unknown_table_using_aliased_import() -> Result<(), Box<dyn std::error::Error>> {
        // SETUP
        let dir = tempdir()?;
        let parent_path = dir.path();
        let file_path = parent_path.join("index.ts");

        let index_content = r#"
import { sql as aliased } from "sqlx-ts";

/////////////////
// expressions //
/////////////////

const query1 = aliased`SELECT * FROM aliased_unknown;`;

///////////////
// functions //
///////////////

function test() {
    const name = "sqlx-ts";
    const query3 = aliased`
        SELECT * FROM aliased_unknown_function;
    `;

    // Following query should fail to compile as it gives more values than available fields
    return aliased`
        INSERT INTO
    items (food_type, time_takes_to_cook, table_id, points)
    VALUES ('steak', 1, 1, 20, 1);
    `;
}
        
        "#;
        let mut temp_file = fs::File::create(&file_path)?;
        writeln!(temp_file, "{}", index_content)?;

        // EXECUTE
        let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();

        cmd.arg(parent_path.to_str().unwrap())
            .arg("--ext=ts")
            .arg("--db-type=mysql")
            .arg("--db-host=127.0.0.1")
            .arg("--db-port=33306")
            .arg("--db-user=root")
            .arg("--db-name=sqlx-ts")
            .arg("-g");

        // ASSERT
        cmd.assert()
            .failure()
            .stderr(predicates::str::contains(
                "Table \'sqlx-ts.aliased_unknown\' doesn\'t exist",
            ))
            .stderr(predicates::str::contains(
                "Table \'sqlx-ts.aliased_unknown_function\' doesn\'t exist",
            ))
            .stderr(predicates::str::contains(
                "Column count doesn\'t match value count at row 1",
            ));
        Ok(())
    }
}
