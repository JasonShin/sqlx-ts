#[cfg(test)]
mod postgres_query_paramters_tests {
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

        Ok(())
    }
}
