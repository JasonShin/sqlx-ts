import { sql } from 'sqlx-ts'

async function asyncFunction() {
    return await sql`
-- @name: async plain function
SELECT id FROM items;
`
}

const asyncLambdaAwaited = async () => await sql`SELECT id FROM items`

const asyncLambda = async () => sql`SELECT id FROM items`

(async function () {
    const iifFunction = sql`SELECT id FROM items;`
})()

(async () => sql`
-- @name: iif lambda
SELECT id FROM items;
`)()
