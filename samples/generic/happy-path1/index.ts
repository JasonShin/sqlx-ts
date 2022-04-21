import { sql as sqlx } from 'sqlx-ts'
/////////////////
// expressions //
/////////////////

const query1 = sqlx`SELECT * FROM items;`
// variable de
const query2 = sqlx`
   SELECT * FROM items;
`

///////////////
// functions //
///////////////

function test() {
    const name = 'sqlx-ts'
    const query3 = sqlx`
        SELECT * FROM items;
    `

    return sqlx`
        INSERT INTO
    items (food_type, time_takes_to_cook, table_id, points)
    VALUES ('sushi', 1, 1, 20, 1);
    `
}
