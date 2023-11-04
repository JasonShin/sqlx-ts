import { sql } from 'sqlx-ts'

const sampleQuery = sql`
SELECT id as some_id FROM items
`
