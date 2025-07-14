use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[cfg(test)]
mod sql_file_tests {
  use super::*;

  #[test]
  fn test_single_sql_file_validation() -> Result<(), Box<dyn std::error::Error>> {
    // SETUP
    let dir = tempdir()?;
    let dir_path = dir.path();
    let sql_file = dir_path.join("users.sql");

    let sql_content = r#"
-- Simple character query matching playpen schema
SELECT id, name, level, experience FROM characters WHERE level > 1;
"#;

    fs::write(&sql_file, sql_content)?;

    // EXECUTE
    let mut cmd = Command::cargo_bin("sqlx-ts")?;
    cmd
      .arg(dir_path.to_str().unwrap())
      .arg("--ext=sql")
      .arg("--db-type=postgres")
      .arg("--db-host=127.0.0.1")
      .arg("--db-port=54321")
      .arg("--db-user=postgres")
      .arg("--db-pass=postgres")
      .arg("--db-name=postgres");

    // ASSERT
    cmd
      .assert()
      .success()
      .stdout(predicates::str::contains("No SQL errors detected!"));

    Ok(())
  }

  #[test]
  fn test_multiple_queries_in_single_sql_file() -> Result<(), Box<dyn std::error::Error>> {
    // SETUP
    let dir = tempdir()?;
    let dir_path = dir.path();
    let sql_file = dir_path.join("multiple_queries.sql");

    let sql_content = r#"
-- @name: getCharacterById
SELECT id, name, level, race_id, class_id FROM characters WHERE id = $1;

-- @name: getCharactersByRace
-- @db: postgres
SELECT id, name, level FROM characters WHERE race_id = $1 ORDER BY level DESC;

-- @name: createCharacter
INSERT INTO characters (name, race_id, class_id, level) VALUES ($1, $2, $3, $4) RETURNING id;
"#;

    fs::write(&sql_file, sql_content)?;

    // EXECUTE
    let mut cmd = Command::cargo_bin("sqlx-ts")?;
    cmd
      .arg(dir_path.to_str().unwrap())
      .arg("--ext=sql")
      .arg("--db-type=postgres")
      .arg("--db-host=127.0.0.1")
      .arg("--db-port=54321")
      .arg("--db-user=postgres")
      .arg("--db-pass=postgres")
      .arg("--db-name=postgres");

    // ASSERT
    cmd
      .assert()
      .success()
      .stdout(predicates::str::contains("Found 3 SQL queries"))
      .stdout(predicates::str::contains("No SQL errors detected!"));

    Ok(())
  }

  #[test]
  fn test_sql_file_type_generation() -> Result<(), Box<dyn std::error::Error>> {
    // SETUP
    let dir = tempdir()?;
    let dir_path = dir.path();
    let sql_file = dir_path.join("character_queries.sql");

    let sql_content = r#"
-- @name: getCharacterById
SELECT id, name, level, experience, gold FROM characters WHERE id = $1;

-- @name: getCharactersByFaction
SELECT c.id, c.name, c.level, r.name as race_name, f.name as faction_name
FROM characters c
JOIN races r ON c.race_id = r.id
JOIN factions f ON r.faction_id = f.id
WHERE f.name = $1;
"#;

    fs::write(&sql_file, sql_content)?;

    // EXECUTE
    let mut cmd = Command::cargo_bin("sqlx-ts")?;
    cmd
      .arg(dir_path.to_str().unwrap())
      .arg("--ext=sql")
      .arg("--db-type=postgres")
      .arg("--db-host=127.0.0.1")
      .arg("--db-port=54321")
      .arg("--db-user=postgres")
      .arg("--db-pass=postgres")
      .arg("--db-name=postgres")
      .arg("-g"); // Generate types

    // ASSERT
    cmd
      .assert()
      .success()
      .stdout(predicates::str::contains("No SQL errors detected!"));

    // Check that types file was generated
    let types_file = dir_path.join("character_queries.queries.ts");
    assert!(types_file.exists());

    let types_content = fs::read_to_string(types_file)?;

    // Verify generated types contain expected interfaces
    assert!(types_content.contains("GetCharacterByIdParams"));
    assert!(types_content.contains("IGetCharacterByIdResult"));
    assert!(types_content.contains("GetCharactersByFactionParams"));
    assert!(types_content.contains("IGetCharactersByFactionResult"));

    Ok(())
  }

  #[test]
  fn test_sql_file_with_complex_queries() -> Result<(), Box<dyn std::error::Error>> {
    // SETUP
    let dir = tempdir()?;
    let dir_path = dir.path();
    let sql_file = dir_path.join("complex_queries.sql");

    let sql_content = r#"
-- @name: getCharactersWithInventory
SELECT
  c.id,
  c.name,
  c.level,
  COUNT(i.id) as item_count,
  SUM(inv.quantity) as total_items
FROM characters c
LEFT JOIN inventory inv ON c.id = inv.character_id
LEFT JOIN items i ON inv.id = i.inventory_id
WHERE c.level >= $1
GROUP BY c.id, c.name, c.level
HAVING COUNT(i.id) > $2
ORDER BY total_items DESC
LIMIT $3;

-- @name: getGuildMembersByRank
SELECT
  g.name as guild_name,
  c.name as character_name,
  gm.rank,
  c.level
FROM guild_members gm
JOIN guilds g ON gm.guild_id = g.id
JOIN characters c ON gm.character_id = c.id
WHERE gm.rank = $1
ORDER BY c.level DESC;
"#;

    fs::write(&sql_file, sql_content)?;

    // EXECUTE
    let mut cmd = Command::cargo_bin("sqlx-ts")?;
    cmd
      .arg(dir_path.to_str().unwrap())
      .arg("--ext=sql")
      .arg("--db-type=postgres")
      .arg("--db-host=127.0.0.1")
      .arg("--db-port=54321")
      .arg("--db-user=postgres")
      .arg("--db-pass=postgres")
      .arg("--db-name=postgres");

    // ASSERT
    cmd
      .assert()
      .success()
      .stdout(predicates::str::contains("Found 2 SQL queries"))
      .stdout(predicates::str::contains("No SQL errors detected!"));

    Ok(())
  }

  #[test]
  fn test_sql_file_with_mysql_syntax() -> Result<(), Box<dyn std::error::Error>> {
    // SETUP
    let dir = tempdir()?;
    let dir_path = dir.path();
    let sql_file = dir_path.join("mysql_queries.sql");

    let sql_content = r#"
-- @name: getCharacterById
SELECT id, name, level, race_id FROM characters WHERE id = ?;

-- @name: getCharactersByLevel
SELECT id, name, level FROM characters WHERE level >= ? ORDER BY level DESC LIMIT ?;

-- @name: createCharacter
INSERT INTO characters (name, race_id, class_id, level) VALUES (?, ?, ?, ?);
"#;

    fs::write(&sql_file, sql_content)?;

    // EXECUTE
    let mut cmd = Command::cargo_bin("sqlx-ts")?;
    cmd
      .arg(dir_path.to_str().unwrap())
      .arg("--ext=sql")
      .arg("--db-type=mysql")
      .arg("--db-host=127.0.0.1")
      .arg("--db-port=33306")
      .arg("--db-user=root")
      .arg("--db-pass=")
      .arg("--db-name=sqlx-ts");

    // ASSERT
    cmd
      .assert()
      .success()
      .stdout(predicates::str::contains("Found 3 SQL queries"))
      .stdout(predicates::str::contains("No SQL errors detected!"));

    Ok(())
  }

  #[test]
  fn test_sql_file_with_comments_and_whitespace() -> Result<(), Box<dyn std::error::Error>> {
    // SETUP
    let dir = tempdir()?;
    let dir_path = dir.path();
    let sql_file = dir_path.join("commented_queries.sql");

    let sql_content = r#"
/*
 * Multi-line comment at the top
 * This should be ignored
 */

-- Single line comment

-- @name: getCharacterData
-- This is a comment that should be preserved
SELECT
  id,           -- Character ID
  name,         -- Character name
  level,        -- Character level
  experience    -- Total experience points
FROM characters
WHERE id = $1;  -- Filter by character ID

/*
 * Another multi-line comment
 */

-- @name: getActiveCharacters
SELECT id, name, level
FROM characters
WHERE level > 1
  AND created_at > NOW() - INTERVAL '30 days'; -- Active in last 30 days
"#;

    fs::write(&sql_file, sql_content)?;

    // EXECUTE
    let mut cmd = Command::cargo_bin("sqlx-ts")?;
    cmd
      .arg(dir_path.to_str().unwrap())
      .arg("--ext=sql")
      .arg("--db-type=postgres")
      .arg("--db-host=127.0.0.1")
      .arg("--db-port=54321")
      .arg("--db-user=postgres")
      .arg("--db-pass=postgres")
      .arg("--db-name=postgres");

    // ASSERT
    cmd
      .assert()
      .success()
      .stdout(predicates::str::contains("Found 2 SQL queries"))
      .stdout(predicates::str::contains("No SQL errors detected!"));

    Ok(())
  }

  #[test]
  fn test_multiple_sql_files_in_directory() -> Result<(), Box<dyn std::error::Error>> {
    // SETUP
    let dir = tempdir()?;
    let dir_path = dir.path();

    // Create multiple SQL files using playpen schema
    let characters_sql = dir_path.join("characters.sql");
    let guilds_sql = dir_path.join("guilds.sql");
    let inventory_sql = dir_path.join("inventory.sql");

    fs::write(&characters_sql, "SELECT * FROM characters WHERE level >= $1;")?;
    fs::write(&guilds_sql, "SELECT * FROM guilds WHERE created_at > $1;")?;
    fs::write(&inventory_sql, "SELECT * FROM inventory WHERE character_id = $1;")?;

    // EXECUTE
    let mut cmd = Command::cargo_bin("sqlx-ts")?;
    cmd
      .arg(dir_path.to_str().unwrap())
      .arg("--ext=sql")
      .arg("--db-type=postgres")
      .arg("--db-host=127.0.0.1")
      .arg("--db-port=54321")
      .arg("--db-user=postgres")
      .arg("--db-pass=postgres")
      .arg("--db-name=postgres");

    // ASSERT
    cmd
      .assert()
      .success()
      .stdout(predicates::str::contains("Found 3 SQL queries"))
      .stdout(predicates::str::contains("No SQL errors detected!"));

    Ok(())
  }

  #[test]
  fn test_sql_file_with_invalid_syntax() -> Result<(), Box<dyn std::error::Error>> {
    // SETUP
    let dir = tempdir()?;
    let dir_path = dir.path();
    let sql_file = dir_path.join("invalid.sql");

    let sql_content = r#"
-- This query has invalid SQL syntax
SELECT id, name, FROM characters WHERE; -- Missing table and incomplete WHERE clause
"#;

    fs::write(&sql_file, sql_content)?;

    // EXECUTE
    let mut cmd = Command::cargo_bin("sqlx-ts")?;
    cmd
      .arg(dir_path.to_str().unwrap())
      .arg("--ext=sql")
      .arg("--db-type=postgres")
      .arg("--db-host=127.0.0.1")
      .arg("--db-port=54321")
      .arg("--db-user=postgres")
      .arg("--db-pass=postgres")
      .arg("--db-name=postgres");

    // ASSERT - Should fail due to invalid SQL
    cmd
      .assert()
      .failure()
      .stderr(predicates::str::contains(
        "error: internal compiler error: syntax error at or near \"FROM\"",
      ))
      .stderr(predicates::str::contains("SQLs failed to compile!"));

    Ok(())
  }

  #[test]
  fn test_sql_file_with_database_connection_annotation() -> Result<(), Box<dyn std::error::Error>> {
    // SETUP
    let dir = tempdir()?;
    let dir_path = dir.path();
    let sql_file = dir_path.join("multi_db.sql");
    let config_file = dir_path.join(".sqlxrc.json");

    let sql_content = r#"
-- @name: getDefaultCharacters
SELECT * FROM characters;

-- @name: getCharactersFromSecondDB
-- @db: secondary
SELECT * FROM characters WHERE level > $1;
"#;

    let config_content = r#"
{
  "connections": {
    "default": {
      "DB_TYPE": "postgres",
      "DB_HOST": "127.0.0.1",
      "DB_PORT": 54321,
      "DB_USER": "postgres",
      "DB_PASS": "postgres",
      "DB_NAME": "postgres"
    },
    "secondary": {
      "DB_TYPE": "postgres",
      "DB_HOST": "127.0.0.1",
      "DB_PORT": 54321,
      "DB_USER": "postgres",
      "DB_PASS": "postgres",
      "DB_NAME": "postgres_secondary"
    }
  }
}
"#;

    fs::write(&sql_file, sql_content)?;
    fs::write(&config_file, config_content)?;

    // EXECUTE
    let mut cmd = Command::cargo_bin("sqlx-ts")?;
    cmd
      .arg(dir_path.to_str().unwrap())
      .arg("--ext=sql")
      .arg(format!("--config={}", config_file.to_str().unwrap()));

    // ASSERT
    cmd
      .assert()
      .success()
      .stdout(predicates::str::contains("Found 2 SQL queries"))
      .stdout(predicates::str::contains("No SQL errors detected!"));

    Ok(())
  }

  #[test]
  fn test_sql_file_type_generation_with_custom_path() -> Result<(), Box<dyn std::error::Error>> {
    // SETUP
    let dir = tempdir()?;
    let dir_path = dir.path();
    let sql_file = dir_path.join("queries.sql");
    let types_dir = dir_path.join("generated");
    let types_file = types_dir.join("types.ts");

    fs::create_dir_all(&types_dir)?;

    let sql_content = r#"
-- @name: getCharacter
SELECT id, name, level, experience FROM characters WHERE id = $1;
"#;

    fs::write(&sql_file, sql_content)?;

    // EXECUTE
    let mut cmd = Command::cargo_bin("sqlx-ts")?;
    cmd
      .arg(dir_path.to_str().unwrap())
      .arg("--ext=sql")
      .arg("--db-type=postgres")
      .arg("--db-host=127.0.0.1")
      .arg("--db-port=54321")
      .arg("--db-user=postgres")
      .arg("--db-pass=postgres")
      .arg("--db-name=postgres")
      .arg("-g")
      .arg(format!("--generate-path={}", types_file.to_str().unwrap()));

    // ASSERT
    cmd
      .assert()
      .success()
      .stdout(predicates::str::contains("No SQL errors detected!"));

    assert!(types_file.exists());
    let types_content = fs::read_to_string(types_file)?;
    assert!(types_content.contains("GetCharacterParams"));
    assert!(types_content.contains("IGetCharacterResult"));

    Ok(())
  }

  #[test]
  fn test_empty_sql_file() -> Result<(), Box<dyn std::error::Error>> {
    // SETUP
    let dir = tempdir()?;
    let dir_path = dir.path();
    let sql_file = dir_path.join("empty.sql");

    fs::write(&sql_file, "")?;

    // EXECUTE
    let mut cmd = Command::cargo_bin("sqlx-ts")?;
    cmd
      .arg(dir_path.to_str().unwrap())
      .arg("--ext=sql")
      .arg("--db-type=postgres")
      .arg("--db-host=127.0.0.1")
      .arg("--db-port=54321")
      .arg("--db-user=postgres")
      .arg("--db-pass=postgres")
      .arg("--db-name=postgres");

    // ASSERT
    cmd
      .assert()
      .success()
      .stdout(predicates::str::contains("No SQL queries found"))
      .stdout(predicates::str::contains("No SQL errors detected!"));

    Ok(())
  }

  #[test]
  fn test_playpen_schema_inventory_items_relationship() -> Result<(), Box<dyn std::error::Error>> {
    // SETUP
    let dir = tempdir()?;
    let dir_path = dir.path();
    let sql_file = dir_path.join("inventory_items.sql");

    let sql_content = r#"
-- @name: getCharacterInventory
-- Query using the correct playpen schema relationship
SELECT
  c.name as character_name,
  i.name as item_name,
  inv.quantity
FROM characters c
JOIN inventory inv ON c.id = inv.character_id
JOIN items i ON inv.id = i.inventory_id
WHERE c.id = $1;

-- @name: getItemsByRarity
SELECT
  i.name,
  i.rarity,
  i.flavor_text
FROM items i
WHERE i.rarity = $1;
"#;

    fs::write(&sql_file, sql_content)?;

    // EXECUTE
    let mut cmd = Command::cargo_bin("sqlx-ts")?;
    cmd
      .arg(dir_path.to_str().unwrap())
      .arg("--ext=sql")
      .arg("--db-type=postgres")
      .arg("--db-host=127.0.0.1")
      .arg("--db-port=54321")
      .arg("--db-user=postgres")
      .arg("--db-pass=postgres")
      .arg("--db-name=postgres");

    // ASSERT
    cmd
      .assert()
      .success()
      .stdout(predicates::str::contains("Found 2 SQL queries"))
      .stdout(predicates::str::contains("No SQL errors detected!"));

    Ok(())
  }
}
