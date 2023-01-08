#[cfg(test)]
mod query_parameters_tests {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::fs;
    use std::io::Write;
    use std::process::Command;
    use tempfile::tempdir;

    use test_utils::test_utils::TSString;

    #[test]
    fn should_pick_query_params_from_flat_list_of_binary_ops() -> Result<(), Box<dyn std::error::Error>> {
        // SETUP
        let dir = tempdir()?;
        let parent_path = dir.path();
        let file_path = parent_path.join("index.ts");

        let index_content = r#"
import { sql } from 'sqlx-ts';

const someQuery = sql`
SELECT *
FROM items
WHERE points > ?
AND points < ?
OR points = ?
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
        cmd.assert().success();

        let type_file = fs::read_to_string(parent_path.join("index.queries.ts"))?;
        let type_file = type_file.trim();
        let gen_query_types = r#"
export type SomeQueryParams = [number, number, number];

export interface ISomeQueryResult {
    food_type: string;
    id: number;
    points: number;
    table_id: number;
    time_takes_to_cook: number;
};

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
};
        "#;
        assert_eq!(
            gen_query_types.trim().to_string().flatten(),
            type_file.to_string().flatten()
        );
        Ok(())
    }

    #[test]
    fn should_pick_query_params_from_in_list() -> Result<(), Box<dyn std::error::Error>> {
        // SETUP
        let dir = tempdir()?;
        let parent_path = dir.path();
        let file_path = parent_path.join("index.ts");

        let index_content = r#"
import { sql } from 'sqlx-ts';

const someQuery = sql`
SELECT *
FROM items
WHERE id IN (?);
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
        cmd.assert().success();

        let type_file = fs::read_to_string(parent_path.join("index.queries.ts"))?;
        let type_file = type_file.trim();
        let gen_query_types = r#"
export type SomeQueryParams = [Array<number>];

export interface ISomeQueryResult {
    food_type: string;
    id: number;
    points: number;
    table_id: number;
    time_takes_to_cook: number;
};

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
};
        "#;
        assert_eq!(
            gen_query_types.trim().to_string().flatten(),
            type_file.to_string().flatten()
        );
        Ok(())
    }

    #[test]
    fn should_pick_query_params_from_subqueries() -> Result<(), Box<dyn std::error::Error>> {
        // SETUP
        let dir = tempdir()?;
        let parent_path = dir.path();
        let file_path = parent_path.join("index.ts");

        let index_content = r#"
import { sql } from 'sqlx-ts';

const someQuery = sql`
SELECT *
FROM items
WHERE id IN (
    SELECT id
    FROM items
    WHERE points > ?
    AND id IN (
        SELECT id
        FROM items
        WHERE food_type = ?
    )
) AND points < ?;
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
        cmd.assert().success();

        let type_file = fs::read_to_string(parent_path.join("index.queries.ts"))?;
        let type_file = type_file.trim();
        let gen_query_types = r#"
export type SomeQueryParams = [number, string, number];

export interface ISomeQueryResult {
    food_type: string;
    id: number;
    points: number;
    table_id: number;
    time_takes_to_cook: number;
};

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
};
        "#;
        assert_eq!(
            gen_query_types.trim().to_string().flatten(),
            type_file.to_string().flatten()
        );
        Ok(())
    }
}
