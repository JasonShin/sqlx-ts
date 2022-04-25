import { sql as aliased } from 'sqlx-ts'

/////////////////
// expressions //
/////////////////

const query1 = aliased`SELECT * FROM aliased_unknown;`

///////////////
// functions //
///////////////

function test() {
    const name = 'sqlx-ts'
    const query3 = aliased`
        SELECT * FROM items;
    `

    // Following query should fail to compile as it gives more values than available fields
    return aliased`
        INSERT INTO
    items (food_type, time_takes_to_cook, table_id, points)
    VALUES ('steak', 1, 1, 20, 1);
    `
}
