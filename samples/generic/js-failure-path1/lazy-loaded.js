// TODO: Support lazy loading in the future phases

function test() {
    const { sql: sqlx } = require('sqlx-ts')

    const someQuery = sqlx`SELECT * FROM unknown`

    return someQuery
}

function test2() {
    const SQLX = require('sqlx-ts')
    const someQuery = SQLX.sql`SELECT * FROM unknown`

    return someQuery
}

test()
test2()
