function sql(query) {
    return query
}

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
    const name = 'sqlx-ts'
    const query3 = sql`
        SELECT * FROM items;
    `

    return sql`
        INSERT INTO
    items (food_type, time_takes_to_cook, table_id, points)
    VALUES ('sushi', 1, aaa, 20);
    `
}
