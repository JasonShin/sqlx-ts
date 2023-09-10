import { sql } from 'sqlx-ts'

function tester(someSql: string) {
    return "Hello World";
}

tester(sql`
-- @name: caller test
SELECT id FROM items WHERE id = 1
`)
