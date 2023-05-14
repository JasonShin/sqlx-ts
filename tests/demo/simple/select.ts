import { sql } from 'sqlx-ts'

// simple select
const sql1 = sql`SELECT * FROM items`

// single join
const sql2 = sql`
SELECT *
FROM items
JOIN tables ON items.table_id = tables.id
`

// subquery
const sql3 = sql`
SELECT
    (SELECT number FROM tables WHERE tables.id = items.table_id) AS table_number
FROM items
`
