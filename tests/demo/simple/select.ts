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

// wildcard
const sql4 = sql`
SELECT items.*
FROM items;
`

// table wit joins
const sql5 = sql`
SELECT tables.*
FROM items
JOIN tables ON items.table_id = tables.id
`

// Various operators

const sql6 = sql`
SELECT id
FROM items
WHERE points BETWEEN $1 AND $2;
`

const sql7 = sql`
SELECT
	id,
	CAST('2015-01-01' AS DATE) as DATE
FROM items;
`
