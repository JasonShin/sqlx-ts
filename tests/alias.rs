#[cfg(test)]
mod alias {
  use assert_cmd::prelude::*;

  use std::fs;
  use std::io::Write;
  use std::process::Command;
  use tempfile::tempdir;

  #[test]
  fn should_warn_on_clashing_field_names_on_join() -> Result<(), Box<dyn std::error::Error>> {
    let ts_content = r#"
const someQuery = sql`
SELECT items.inventory_id, items.id, inventory.id
FROM items
JOIN inventory ON items.inventory_id = inventory.id
`"#;

    // SETUP
    let dir = tempdir()?;
    let parent_path = dir.path();
    let file_path = parent_path.join("index.ts");
    let mut temp_file = fs::File::create(file_path)?;
    writeln!(temp_file, "{}", ts_content)?;

    // EXECUTE
    let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();
    cmd
      .arg(parent_path.to_str().unwrap())
      .arg("--ext=ts")
      .arg("--config=.sqlxrc.sample.json")
      .arg("-g");

    // ASSERT
    cmd.assert().success()
       .stdout(predicates::str::contains("Missing an alias for a compound identifier, using items_inventory_id as the key name. Prefer adding an alias for example: `items.inventory_id AS inventory_id`"))
       .stdout(predicates::str::contains("Missing an alias for a compound identifier, using items_id as the key name. Prefer adding an alias for example: `items.id AS id`"))
       .stdout(predicates::str::contains("Missing an alias for a compound identifier, using inventory_id as the key name. Prefer adding an alias for example: `inventory.id AS id`"))
      .stdout(predicates::str::contains("No SQL errors detected!"));

    Ok(())
  }

  #[test]
  #[should_panic(
    expected = "Impossible to calculate appropriate field names of a wildcard query with multiple tables."
  )]
  fn should_not_warn_on_field_names_of_asterix() {
    let ts_content = r#"
        const someQuery = sql`
        SELECT *
        FROM items
        `"#;

    // SETUP
    let dir = tempdir().unwrap();
    let parent_path = dir.path();
    let file_path = parent_path.join("index.ts");
    let mut temp_file = fs::File::create(file_path).unwrap();
    writeln!(temp_file, "{}", ts_content).unwrap();

    // EXECUTE
    let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();
    cmd
      .arg(parent_path.to_str().unwrap())
      .arg("--ext=ts")
      .arg("--config=.sqlxrc.sample.json")
      .arg("-g");

    // ASSERT
    cmd
      .assert()
      .success()
      // check not condition
      .try_stdout(predicates::str::contains(
        "Impossible to calculate appropriate field names of a wildcard query with multiple tables.",
      ))
      .unwrap();
  }

  #[test]
  fn should_warn_on_clashing_field_names_on_join_asterix() -> Result<(), Box<dyn std::error::Error>> {
    let ts_content = r#"
        const someQuery = sql`
        SELECT *
        FROM items
        JOIN inventory ON items.inventory_id = inventory.id
        `"#;

    // SETUP
    let dir = tempdir()?;
    let parent_path = dir.path();
    let file_path = parent_path.join("index.ts");
    let mut temp_file = fs::File::create(file_path)?;
    writeln!(temp_file, "{}", ts_content)?;

    // EXECUTE
    let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();
    cmd
      .arg(parent_path.to_str().unwrap())
      .arg("--ext=ts")
      .arg("--config=.sqlxrc.sample.json")
      .arg("-g");

    // ASSERT
    cmd
      .assert()
      .success()
      .stdout(predicates::str::contains(
        "Impossible to calculate appropriate field names of a wildcard query with multiple tables.",
      ))
      .stdout(predicates::str::contains("No SQL errors detected!"));

    Ok(())
  }
}
