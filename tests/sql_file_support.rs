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
-- Simple user query
SELECT id, name, email FROM users WHERE active = true;
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
-- @name: getUserById
SELECT id, name, email FROM users WHERE id = $1;

-- @name: getUsersByStatus
-- @db: postgres
SELECT id, name, email FROM users WHERE status = $1 ORDER BY created_at DESC;

-- @name: createUser
INSERT INTO users (name, email, status) VALUES ($1, $2, $3) RETURNING id;
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
    let sql_file = dir_path.join("user_queries.sql");

    let sql_content = r#"
-- @name: getUserById
SELECT id, name, email, created_at FROM users WHERE id = $1;

-- @name: getUsersByEmail
SELECT id, name, email FROM users WHERE email = $1;
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
    let types_file = dir_path.join("user_queries.queries.ts");
    assert!(types_file.exists());

    let types_content = fs::read_to_string(types_file)?;

    // Verify generated types contain expected interfaces
    assert!(types_content.contains("IGetUserByIdParams"));
    assert!(types_content.contains("IGetUserByIdResult"));
    assert!(types_content.contains("IGetUsersByEmailParams"));
    assert!(types_content.contains("IGetUsersByEmailResult"));

    Ok(())
  }

  #[test]
  fn test_sql_file_with_complex_queries() -> Result<(), Box<dyn std::error::Error>> {
    // SETUP
    let dir = tempdir()?;
    let dir_path = dir.path();
    let sql_file = dir_path.join("complex_queries.sql");

    let sql_content = r#"
-- @name: getUsersWithOrders
SELECT
  u.id,
  u.name,
  u.email,
  COUNT(o.id) as order_count,
  SUM(o.total) as total_spent
FROM users u
LEFT JOIN orders o ON u.id = o.user_id
WHERE u.status = $1
GROUP BY u.id, u.name, u.email
HAVING COUNT(o.id) > $2
ORDER BY total_spent DESC
LIMIT $3;

-- @name: getOrdersByDateRange
SELECT
  o.id,
  o.total,
  o.created_at,
  u.name as user_name
FROM orders o
JOIN users u ON o.user_id = u.id
WHERE o.created_at BETWEEN $1 AND $2
ORDER BY o.created_at DESC;
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
-- @name: getUserById
SELECT id, name, email FROM users WHERE id = ?;

-- @name: getUsersByStatus
SELECT id, name, email FROM users WHERE status = ? ORDER BY created_at DESC LIMIT ?;

-- @name: createUser
INSERT INTO users (name, email, status) VALUES (?, ?, ?);
"#;

    fs::write(&sql_file, sql_content)?;

    // EXECUTE
    let mut cmd = Command::cargo_bin("sqlx-ts")?;
    cmd
      .arg(dir_path.to_str().unwrap())
      .arg("--ext=sql")
      .arg("--db-type=mysql")
      .arg("--db-host=127.0.0.1")
      .arg("--db-port=3306")
      .arg("--db-user=root")
      .arg("--db-name=test");

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

-- @name: getUserData
-- This is a comment that should be preserved
SELECT
  id,           -- User ID
  name,         -- Full name
  email,        -- Email address
  created_at    -- Account creation date
FROM users
WHERE id = $1;  -- Filter by user ID

/*
 * Another multi-line comment
 */

-- @name: getActiveUsers
SELECT id, name, email
FROM users
WHERE status = 'active'
  AND deleted_at IS NULL; -- Exclude soft deleted users
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

    // Create multiple SQL files
    let users_sql = dir_path.join("users.sql");
    let orders_sql = dir_path.join("orders.sql");
    let products_sql = dir_path.join("products.sql");

    fs::write(&users_sql, "SELECT * FROM users WHERE id = $1;")?;
    fs::write(&orders_sql, "SELECT * FROM orders WHERE user_id = $1;")?;
    fs::write(&products_sql, "SELECT * FROM products WHERE category = $1;")?;

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
      .stdout(predicates::str::contains("Found 3 SQL files to process"))
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
SELECT id, name, FROM users WHERE; -- Missing table and incomplete WHERE clause
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
      .stdout(predicates::str::contains("SQLs failed to compile!"));

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
-- @name: getDefaultUsers
SELECT * FROM users;

-- @name: getOrdersFromSecondDB
-- @db: secondary
SELECT * FROM orders WHERE status = $1;
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
-- @name: getUser
SELECT id, name, email FROM users WHERE id = $1;
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
    assert!(types_content.contains("IGetUserParams"));
    assert!(types_content.contains("IGetUserResult"));

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
}
