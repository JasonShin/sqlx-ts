import { sql } from 'sqlx-ts'

const sql1 = sql`
DELETE FROM items
WHERE id = $1
`
const sql2 = sql`
DELETE FROM "items"
WHERE "id" = $1
`
