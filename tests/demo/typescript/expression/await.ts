import { sql } from 'sqlx-ts'

async function asyncFunction() {
    return await sql`
-- @name: async plain function
SELECT id FROM items;
`
}

// lambda
// ffi
