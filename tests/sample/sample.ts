import { sql } from 'sqlx-ts'

const sampleSelectQuery = sql`
SELECT id as some_id, name FROM items WHERE id = $1
`

const sampleInsertQuery = sql`
INSERT INTO items (name) VALUES ($1)
`

const sampleUpdateQuery = sql`
UPDATE items SET name = $1 WHERE id = $2
`

const sampleDeleteQuery = sql`
DELETE FROM items WHERE id = $1
`
