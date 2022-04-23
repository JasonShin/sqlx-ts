import { sql as aliasedSql } from 'sqlx-ts'

/////////////////
// expressions //
/////////////////

const query1 = aliasedSql`SELECT * FROM items;`
// variable de
const query2 = aliasedSql`
   SELECT * FROM items;
`

///////////////
// functions //
///////////////

function test() {
    const name = 'sqlx-ts'
    const query3 = aliasedSql`
        SELECT * FROM items;
    `

    return aliasedSql`
        INSERT INTO
    items (food_type, time_takes_to_cook, table_id, points)
    VALUES ('sushi', 1, aaa, 20);
    `
}
