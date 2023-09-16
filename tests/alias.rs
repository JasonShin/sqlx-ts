#[cfg(test)]
mod alias {
    use assert_cmd::prelude::*;
    use pretty_assertions::assert_eq;
    use std::env::current_dir;
    use std::fs;
    use std::io::Write;
    use std::process::Command;
    use tempfile::tempdir;
    use walkdir::WalkDir;

    #[test]
    fn should_warn_on_clashing_field_names_on_join() -> Result<(), Box<dyn std::error::Error>> {
        let ts_content = r#"
const someQuery = sql`
SELECT items.table_id, items.id, tables.id
FROM items
JOIN tables ON items.table_id = tables.id
`
        "#;

        // SETUP
        let dir = tempdir()?;
        let parent_path = dir.path();
        let file_path = parent_path.join(format!("index.ts"));
        let mut temp_file = fs::File::create(&file_path)?;
        writeln!(temp_file, "{}", ts_content)?;

        // EXECUTE
        let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();
        cmd.arg(parent_path.to_str().unwrap())
            .arg("--ext=ts")
            .arg("--config=.sqlxrc.sample.json")
            .arg("-g");

        // ASSERT
        cmd.assert().success()
       .stdout(predicates::str::contains("Missing an alias for a compound identifier, using items_table_id as the key name. Prefer adding an alias for example: `items.table_id AS table_id`"))
       .stdout(predicates::str::contains("Missing an alias for a compound identifier, using items_id as the key name. Prefer adding an alias for example: `items.id AS id`"))
       .stdout(predicates::str::contains("Missing an alias for a compound identifier, using tables_id as the key name. Prefer adding an alias for example: `tables.id AS id`"))
      .stdout(predicates::str::contains("No SQL errors detected!"));

        Ok(())
    }
}
