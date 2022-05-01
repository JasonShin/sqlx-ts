import { sql } from 'sqlx-ts'
/////////////////
// expressions //
/////////////////

const query1 = sql`SELECT * FROM items;`
// variable de
const query2 = sql`
   SELECT * FROM items;
`

///////////////
// functions //
///////////////
function test() {
    const query3 = sql`
        SELECT * FROM items;
    `

    return sql`
        INSERT INTO
    items (food_type, time_takes_to_cook, table_id, points)
    VALUES ('steak', 1, 1, 20);
    `
}

///////////////////
// If statements //
///////////////////
if (true) {
    const query3 = sql`SELECT * FROM items;`
}

function testIfStatement() {
    if (true) {
        const query3 = sql`SELECT * FROM items;`
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
