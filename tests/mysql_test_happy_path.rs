const TEST_INDEX_CONTENT_1: &str = r#"
import { sql } from "sqlx-ts";
/////////////////
// expressions //
/////////////////

const query0 = sql`
SELECT id, points
FROM items;
`;

const queryCompount = sql`
SELECT items.id, tables.id
FROM items
JOIN tables ON items.table_id = tables.id;
`;

const query1 = sql`SELECT * FROM items;`;
// variable de
const query2 = sql`
   SELECT * FROM items;
`;

///////////////
// functions //
///////////////
function test() {
  const query3 = sql`
        SELECT * FROM items;
    `;

  return sql`
    -- @name: testQuery
        INSERT INTO
    items (food_type, time_takes_to_cook, table_id, points)
    VALUES ('steak', 1, 1, 20);
    `;
}

///////////////////
// If statements //
///////////////////
if (true) {
  const query3 = sql`SELECT * FROM items;`;
}

function testIfStatement() {
  if (true) {
    const query3 = sql`SELECT * FROM items;`;
  }
}

//////////////////////
// Switch Statement //
//////////////////////

switch (true) {
  case true:
    const query4 = sql`SELECT * FROM items`;
    break;
  default:
    const query5 = sql`SELECT * FROM items`;
}

///////////////
// For loops //
///////////////

for (let i = 0; i < 10; i++) {
  const query3 = sql`SELECT * FROM items`;
}

const list = [1, 2, 3];
for (let n in list) {
  const query3 = sql`SELECT * FROM items`;
}

for (let n of list) {
  const query3 = sql`SELECT * FROM items`;
}

///////////////
// Try/Catch //
///////////////

try {
  const query3 = sql`SELECT * FROM items`;
} catch {
  const query3 = sql`SELECT * FROM items`;

  throw sql`
    -- @name: testQuery
    SELECT * FROM items
    `;
}

////////////////////
// with statement //
////////////////////

function with_stmt(o: string, n: number) {
  // @ts-ignore
with (o) {
        const query3 = sql`SELECT * FROM items`;
    }
}

/////////////////////
// While Statement //
/////////////////////

let i = 0;
while (i < 5) {
  const query = sql`SELECT * FROM items`;
  i++;
}

i = 0;
do {
  const query = sql`SELECT * FROM items`;
  i++;
} while (i < 5);

///////////
// Class //
///////////

class Foo {
  private bar() {
    const query = sql`SELECT * FROM items`;
  }

  public baz() {
    const query = sql`SELECT * FROM items`;
  }

  protected qux() {
    const query = sql`SELECT * FROM items`;
  }
}

///////////////////////////////////
// Interface, type, enum, module //
///////////////////////////////////

interface TestInterface {
  name: string;
}

type TestType = number;

enum TestEnum {
  a,
  b,
  c,
}

module TestModule {
}
"#;

#[cfg(test)]
mod mysql_happy_path_tests {
    use crate::TEST_INDEX_CONTENT_1;
    use assert_cmd::prelude::*;
    use std::fs;
    use std::io::Write;
    use std::process::Command;
    use tempfile::tempdir;

    macro_rules! success_with_all_cli_args {
($($name:ident: $value:expr,)*) => {
$(
// MACRO STARTS


  #[test]
  fn $name() -> Result<(), Box<dyn std::error::Error>> {
      let ts_type = $value;
      // SETUP
      let dir = tempdir()?;
      let parent_path = dir.path();
      let file_path = parent_path.join(format!("index.{ts_type}"));

      let mut temp_file = fs::File::create(&file_path)?;
      writeln!(temp_file, "{}", TEST_INDEX_CONTENT_1)?;

      // EXECUTE
      let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();

      cmd.arg(parent_path.to_str().unwrap())
          .arg(format!("--ext={ts_type}"))
          .arg("--db-type=mysql")
          .arg("--db-host=127.0.0.1")
          .arg("--db-port=33306")
          .arg("--db-user=root")
          .arg("--db-name=sqlx-ts")
          .arg("-g");

      // ASSERT
      cmd.assert()
          .success()
          .stdout(predicates::str::contains("No SQL errors detected!"));

      Ok(())
  }


// MACRO ENDS
)*};}

    success_with_all_cli_args! {
      js_success_with_all_cli_args: "js",
      ts_success_with_all_cli_args: "ts",
    }

    macro_rules! success_with_env_vars {
($($name:ident: $value:expr,)*) => {
$(
// MACRO STARTS

  /// should not be using any arg to provide credential for DB connection
  #[test]
  fn $name() -> Result<(), Box<dyn std::error::Error>> {
      let ts_type = $value;
      // SETUP
      let dir = tempdir()?;
      let parent_path = dir.path();
      let file_path = parent_path.join(format!("index.{ts_type}"));

      let mut temp_file = fs::File::create(&file_path)?;
      writeln!(temp_file, "{}", TEST_INDEX_CONTENT_1)?;

      // EXECUTE
      let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();

      cmd.env("DB_HOST", "127.0.0.1")
          .env("DB_PORT", "33306")
          .env("DB_USER", "root")
          .env("DB_NAME", "sqlx-ts");
      cmd.arg(parent_path.to_str().unwrap())
          .arg(format!("--ext={ts_type}"))
          .arg("--db-type=mysql");

      // ASSERT
      cmd.assert()
          .success()
          .stdout(predicates::str::contains("No SQL errors detected!"));

      Ok(())
  }


// MACRO ENDS
)*};}

    success_with_env_vars! {
      js_success_with_env_vars: "js",
      ts_success_with_env_vars: "ts",
    }

    macro_rules! sucess_with_partial_env_vars {
($($name:ident: $value:expr,)*) => {
$(
// MACRO STARTS

  #[test]
  fn $name() -> Result<(), Box<dyn std::error::Error>> {
    let ts_type = $value;
    // SETUP
    let dir = tempdir()?;
    let parent_path = dir.path();
    let file_path = parent_path.join(format!("index.{ts_type}"));

    let mut temp_file = fs::File::create(&file_path)?;
    writeln!(temp_file, "{}", TEST_INDEX_CONTENT_1)?;

    // EXECUTE
    let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();

    cmd.env("DB_HOST", "127.0.0.1")
        .env("DB_PORT", "33306")
        .env("DB_USER", "root")
        .env("DB_NAME", "wrong-db");
    cmd.arg(parent_path.to_str().unwrap())
        .arg(format!("--ext={ts_type}"))
        .arg("--db-port=33306")
        .arg("--db-type=mysql")
        .arg("--db-name=sqlx-ts");

    // ASSERT
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("No SQL errors detected!"));

    Ok(())
  }

// MACRO ENDS
)*};}

    sucess_with_partial_env_vars! {
      js_sucess_with_partial_env_vars: "js",
      ts_sucess_with_partial_env_vars: "ts",
    }
}
