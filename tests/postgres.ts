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
}
