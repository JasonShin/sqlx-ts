#[cfg(test)]
mod postgres_insert_query_parameters {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use pretty_assertions::assert_eq;
    use std::fs;
    use std::io::Write;
    use std::process::Command;
    use tempfile::tempdir;

    use test_utils::test_utils::TSString;

    #[test]
    fn should_pick_query_params_from_single_row_of_values() -> Result<(), Box<dyn std::error::Error>> {
        // SETUP
        let dir = tempdir()?;
        let parent_path = dir.path();
        let file_path = parent_path.join("index.ts");

        let index_content = r#"
import { sql } from "sqlx-ts";

const someInputQuery = sql`
INSERT INTO items (id, food_type, time_takes_to_cook, table_id, points)
VALUES
($2, $1, 2, $3, 2);
`
        "#;
        let mut temp_file = fs::File::create(file_path)?;
        writeln!(temp_file, "{}", index_content)?;

        // EXECUTE
        let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();

        cmd.arg(parent_path.to_str().unwrap())
            .arg("--ext=ts")
            .arg("--db-type=postgres")
            .arg("--db-host=127.0.0.1")
            .arg("--db-port=54321")
            .arg("--db-user=postgres")
            .arg("--db-pass=postgres")
            .arg("--db-name=postgres")
            .arg("-g");

        // ASSERT
        cmd.assert().success();

        let type_file = fs::read_to_string(parent_path.join("index.queries.ts"))?;
        let type_file = type_file.trim();
        let gen_query_types = r#"
export type SomeInputQueryParams = [string, number, number];

export interface ISomeInputQueryResult {
    
};

export interface ISomeInputQueryQuery {
    params: SomeInputQueryParams;
    result: ISomeInputQueryResult;
};
        "#;

        assert_eq!(
            gen_query_types.trim().to_string().flatten(),
            type_file.to_string().flatten()
        );
        Ok(())
    }

    #[test]
    fn should_pick_query_params_from_multiple_rows_of_values() -> Result<(), Box<dyn std::error::Error>> {
        // SETUP
        let dir = tempdir()?;
        let parent_path = dir.path();
        let file_path = parent_path.join("index.ts");

        let index_content = r#"
import { sql } from "sqlx-ts";

const someInputQuery = sql`
INSERT INTO items (id, food_type, time_takes_to_cook, table_id, points)
VALUES
($2, $1, 2, $3, 2),
($5, 'test', $4, $7, $6);
`
        "#;
        let mut temp_file = fs::File::create(file_path)?;
        writeln!(temp_file, "{}", index_content)?;

        // EXECUTE
        let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();

        cmd.arg(parent_path.to_str().unwrap())
            .arg("--ext=ts")
            .arg("--db-type=postgres")
            .arg("--db-host=127.0.0.1")
            .arg("--db-port=54321")
            .arg("--db-user=postgres")
            .arg("--db-pass=postgres")
            .arg("--db-name=postgres")
            .arg("-g");

        // ASSERT
        cmd.assert().success();

        let type_file = fs::read_to_string(parent_path.join("index.queries.ts"))?;
        let type_file = type_file.trim();
        let gen_query_types = r#"
export type SomeInputQueryParams = [string, number, number, number, number, number, number];

export interface ISomeInputQueryResult {
    
};

export interface ISomeInputQueryQuery {
    params: SomeInputQueryParams;
    result: ISomeInputQueryResult;
};
        "#;

        assert_eq!(
            gen_query_types.trim().to_string().flatten(),
            type_file.to_string().flatten()
        );
        Ok(())
    }
}
