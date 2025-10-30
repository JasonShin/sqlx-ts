import { sql } from 'sqlx-ts'

// simple select
const selectSql1 = sql`SELECT * FROM items`

// single join
const selectSql2 = sql`
SELECT *
FROM items
JOIN inventory ON items.inventory_id = inventory.id
`

// subquery
const selectSql3 = sql`
SELECT
    (SELECT quantity FROM inventory WHERE inventory.id = items.inventory_id) AS inventory_quantity
FROM items
`

// wildcard
const selectSql4 = sql`
SELECT items.*
FROM items;
`

// table wit joins
const selectSql5 = sql`
SELECT inventory.*
FROM items
JOIN inventory ON items.inventory_id = inventory.id
`

// Various operators
const selectSql6 = sql`
SELECT id
FROM inventory
WHERE quantity BETWEEN $1 AND $2;
`

// Where condition expressions
const selectSql9 = sql`
SELECT *
FROM items
WHERE $1;
`

const selectSql10 = sql`
SELECT *
FROM quests
WHERE completed IS TRUE;
`

// IS DISTINCT FROM operator as part of the WHERE statement
const selectSql11 = sql`
SELECT id, quantity, id IS DISTINCT FROM $1 AS hmm
FROM inventory
WHERE id IS DISTINCT FROM $2
`

const selectSql12 = sql`
SELECT id
FROM inventory
WHERE inventory.id = $1
`

const selectSql13 = sql`
SELECT *
FROM items
WHERE name LIKE $1
`

// SELECT with quoted table names
const selectSql14 = sql`
SELECT * FROM "items" WHERE id = $1
`

const selectSql15 = sql`
SELECT "items"."id", "inventory"."id" AS "id2", inventory.id
FROM "items"
JOIN "inventory" ON "items"."inventory_id" = "inventory"."id"
`

// SELECT IFNULL
const selectSql16 = sql`
SELECT IFNULL(name, 'defaultName') AS name FROM factions;
`
