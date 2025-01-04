import { sql } from 'sqlx-ts'

const deleteSql1 = sql`
DELETE FROM items
WHERE id = $1
`
const deleteSql2 = sql`
DELETE FROM "items"
WHERE "id" = $1
`

const deleteSql3 = sql`
DELETE FROM "items"
WHERE "id" = $1 AND name = $2
`
