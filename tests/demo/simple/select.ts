import { sql } from 'sqlx-ts'

/*
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

const sql8 = sql`
SELECT
    id,
    EXTRACT(MONTH FROM DATE '2017-08-08') AS THE_MONTH
FROM items;
`

// Where condition expressions
const sql9 = sql`
SELECT *
FROM items
WHERE $1;
`

const sql10 = sql`
SELECT *
FROM tables
WHERE occupied IS TRUE;
`

// IS DISTINCT FROM operator as part of the WHERE statement
const sql11 = sql`
SELECT id, number, id IS DISTINCT FROM $1 AS hmm
FROM tables
WHERE id IS DISTINCT FROM $2
`

const sql12 = sql`
SELECT id
FROM tables
WHERE tables.id = $1
`

const sql13 = sql`
SELECT *
FROM items
WHERE food_type LIKE $1
`

const sql14 = sql`
SELECT
    id,
    '2018-09-02 07:09:19'::timestamp AT TIME ZONE 'America/Chicago' as some_date
FROM items;
`

const sql15 = sql`
SELECT
    id,
    CEIL(51.11) AS some_number
FROM items;
`
*/

const someDeleteQuery = sql`
DELETE FROM items
WHERE id = $1
AND time_takes_to_cook > 1
OR food_type = $2;
`
