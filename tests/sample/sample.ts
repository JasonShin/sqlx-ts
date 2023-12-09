import { sql } from 'sqlx-ts'

const sampleSelectQuery = sql`
SELECT id as some_id, points FROM items WHERE id = $1
`

const sampleInsertQuery = sql`
INSERT INTO items (points) VALUES ($1)
`

const sampleUpdateQuery = sql`
UPDATE items SET points = $1 WHERE id = $2
`

const sampleDeleteQuery = sql`
DELETE FROM items WHERE id = $1
`
