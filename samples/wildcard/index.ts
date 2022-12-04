import { sql } from 'sqlx-ts'

const wildcardQuery = sql`
SELECT *
FROM items
JOIN tables ON tables.id = items.table_id;
`
