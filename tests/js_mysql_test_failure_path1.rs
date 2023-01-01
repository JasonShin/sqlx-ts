#[cfg(test)]
mod js_mysql_failure_path_tests {
    use assert_cmd::prelude::*;

    use std::fs;
    use std::io::Write;
    use std::process::Command;
    use tempfile::tempdir;

    #[test]
    fn failure_with_all_cli_args() -> Result<(), Box<dyn std::error::Error>> {
        // SETUP
        let dir = tempdir()?;
        let parent_path = dir.path();
        let file_path = parent_path.join("index.js");

        let index_content = r#"
import { sql } from "sqlx-ts";

// Querying from an unknown table
const someQuery = sql`SELECT * FROM indexjs_unknown`;

// Inserting more values than expected
const insertQuery = sql`
    INSERT INTO items (food_type, time_takes_to_cook, table_id, points)
    VALUES ('steak', 1, 1, 1, 1);
`;

///////////////////
// If statements //
///////////////////
if (true) {
    const query3 = sql`SELECT * FROM if_statement1;`;
}

function testIfStatement() {
    if (true) {
    const query3 = sql`SELECT * FROM if_statement2;`;
    }
}

//////////////////////
// Switch Statement //
//////////////////////

switch (true) {
    case true:
    const query4 = sql`SELECT * FROM switch_statements1`;
    break;
    default:
    const query5 = sql`SELECT * FROM switch_statements2`;
}

///////////////
// For loops //
///////////////

for (let i = 0; i < 10; i++) {
    const query3 = sql`SELECT * FROM for_loops1`;
}

const list = [1, 2, 3];
for (let n in list) {
    const query3 = sql`SELECT * FROM for_loops2`;
}

for (let n of list) {
    const query3 = sql`SELECT * FROM for_loops3`;
}

///////////////
// Try/Catch //
///////////////

try {
    const query3 = sql`SELECT * FROM try1`;
} catch {
    const query3 = sql`SELECT * FROM catch1`;

    throw sql`SELECT * FROM throw1`;
}

/////////////////////
// While Statement //
/////////////////////

let i = 0;
while (i < 5) {
    const query = sql`SELECT * FROM while1`;
    i++;
}

i = 0;
do {
    const query = sql`SELECT * FROM do_while1`;
    i++;
} while (i < 5);
        
        "#;
        let mut temp_file = fs::File::create(&file_path)?;
        writeln!(temp_file, "{}", index_content)?;

        // EXECUTE
        let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();

        cmd.arg(parent_path.to_str().unwrap())
            .arg("--ext=js")
            .arg("--db-type=mysql")
            .arg("--db-host=127.0.0.1")
            .arg("--db-port=33306")
            .arg("--db-user=root")
            .arg("--db-name=sqlx-ts");

        // ASSERT
        cmd.assert()
            .failure()
            .stderr(predicates::str::contains(
                "Table 'sqlx-ts.indexjs_unknown' doesn't exist",
            ))
            .stderr(predicates::str::contains(
                "Column count doesn't match value count at row 1",
            ))
            // src/index.ts -> if statements
            .stderr(predicates::str::contains("Table 'sqlx-ts.if_statement1' doesn't exist"))
            .stderr(predicates::str::contains("Table 'sqlx-ts.if_statement2' doesn't exist"))
            // src/index.ts -> switch statements
            .stderr(predicates::str::contains(
                "Table 'sqlx-ts.switch_statements1' doesn't exist",
            ))
            .stderr(predicates::str::contains(
                "Table 'sqlx-ts.switch_statements2' doesn't exist",
            ))
            // src/index.ts -> for loop statements
            .stderr(predicates::str::contains("Table 'sqlx-ts.for_loops1' doesn't exist"))
            .stderr(predicates::str::contains("Table 'sqlx-ts.for_loops2' doesn't exist"))
            .stderr(predicates::str::contains("Table 'sqlx-ts.for_loops3' doesn't exist"))
            // src/index.ts -> try catch statements
            .stderr(predicates::str::contains("Table 'sqlx-ts.try1' doesn't exist"))
            .stderr(predicates::str::contains("Table 'sqlx-ts.catch1' doesn't exist"))
            .stderr(predicates::str::contains("Table 'sqlx-ts.throw1' doesn't exist"))
            // src/index.ts -> while statement
            .stderr(predicates::str::contains("Table 'sqlx-ts.while1' doesn't exist"))
            // src/index.ts -> do while statement
            .stderr(predicates::str::contains("Table 'sqlx-ts.do_while1' doesn't exist"))
            .stderr(predicates::str::contains("SQLs failed to compile!"));
        Ok(())
    }

    #[test]
    fn fails_to_find_an_unknown_table_using_aliased_import() -> Result<(), Box<dyn std::error::Error>> {
        // SETUP
        let dir = tempdir()?;
        let parent_path = dir.path();
        let file_path = parent_path.join("index.js");

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
            .arg("--ext=js")
            .arg("--db-type=mysql")
            .arg("--db-host=127.0.0.1")
            .arg("--db-port=33306")
            .arg("--db-user=root")
            .arg("--db-name=sqlx-ts")
            .arg("-g");

        // ASSERT
        cmd.assert()
            .failure()
            // src/import-alias.ts
            .stderr(predicates::str::contains(
                "Table 'sqlx-ts.aliased_unknown' doesn't exist",
            ));
        Ok(())
    }
}
