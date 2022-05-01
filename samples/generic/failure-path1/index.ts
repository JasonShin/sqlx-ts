import { sql } from 'sqlx-ts'

// Querying from an unknown table
const someQuery = sql`SELECT * FROM indexjs_unknown`;

// Inserting more values than expected
const insertQuery = sql`
    INSERT INTO items (food_type, time_takes_to_cook, table_id, points)
    VALUES ('steak', 1, 1, 1, 1);
`

///////////////////
// If statements //
///////////////////
if (true) {
    const query3 = sql`SELECT * FROM if_statement1;`
}

function testIfStatement() {
    if (true) {
        const query3 = sql`SELECT * FROM if_statement2;`
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
