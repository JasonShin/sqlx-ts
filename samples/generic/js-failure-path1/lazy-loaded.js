// TODO: Support lazy loading in the future phases

function test() {
    const { sql: sqlx } = require('node')

    const someQuery = sqlx`SELECT * FROM unknown`

    return someQuery
}

function test2() {
    const SQLX = require('node')
    const someQuery = SQLX.sql`SELECT * FROM uknown`

    return someQuery
}

test()
test2()
