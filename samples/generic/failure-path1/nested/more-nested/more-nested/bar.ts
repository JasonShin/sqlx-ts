import { sql } from 'sqlx-ts'

/////////////////
// expressions //
/////////////////

const query1 = sql`SELECT * FROM items;`
const query2 = sql`
   SELECT * FROM items;
`

///////////////
// functions //
///////////////

function test() {
    const name = 'sqlx-ts'
    const query3 = sql`
        SELECT * FROM unknown;
    `

    return sql`
        INSERT INTO
    items (food_type, time_takes_to_cook, table_id, points)
    VALUES ('sushi', 1, 1, 20, 1);
    `
}
