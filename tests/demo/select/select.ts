import { sql } from 'sqlx-ts'

// simple select
const selectSql1 = sql`SELECT * FROM items`

// single join
const selectSql2 = sql`
SELECT *
FROM items
JOIN tables ON items.table_id = tables.id
`

// subquery
const selectSql3 = sql`
SELECT
    (SELECT number FROM tables WHERE tables.id = items.table_id) AS table_number
FROM items
`

// wildcard
const selectSql4 = sql`
SELECT items.*
FROM items;
`

// table wit joins
const selectSql5 = sql`
SELECT tables.*
FROM items
JOIN tables ON items.table_id = tables.id
`

// Various operators
const selectSql6 = sql`
SELECT id
FROM items
WHERE points BETWEEN $1 AND $2;
`

// Where condition expressions
const selectSql9 = sql`
SELECT *
FROM items
WHERE $1;
`

const selectSql10 = sql`
SELECT *
FROM tables
WHERE occupied IS TRUE;
`

// IS DISTINCT FROM operator as part of the WHERE statement
const selectSql11 = sql`
SELECT id, number, id IS DISTINCT FROM $1 AS hmm
FROM tables
WHERE id IS DISTINCT FROM $2
`

const selectSql12 = sql`
SELECT id
FROM tables
WHERE tables.id = $1
`

const selectSql13 = sql`
SELECT *
FROM items
WHERE food_type LIKE $1
`

// SELECT with quoted table names
const selectSql14 = sql`
SELECT * FROM "items" WHERE id = $1
`

const selectSql15 = sql`
SELECT "items"."id", "tables"."id" AS "id2", items.id
FROM "items"
JOIN "tables" ON "items"."table_id" = "tables"."id"
`
