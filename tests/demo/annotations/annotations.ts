import { sql } from 'sqlx-ts'

const annotationSql1 = sql`
-- @name: test mysql query
-- @db: db_mysql
SELECT *
FROM items;
`

const annotationSql2 = sql`
-- @name: test postgres query
-- @db: default
SELECT *
FROM items;
`

const annotationSql3 = sql`
-- @result id: string
SELECT
    id
FROM items;
`

const annotationSql4 = sql`
-- @param 1: string
SELECT *
FROM inventory
WHERE quantity > $1
`

const annotationSql5 = sql`
-- @result inventory_id: boolean
-- @param 1: string
SELECT
    inventory.id AS inventory_id
FROM inventory
JOIN items ON items.inventory_id = inventory.id
WHERE inventory.id = $1
`

const annotationSql6 = sql`
-- @name: test mysql query with param overrides
-- @db: db_mysql
-- @param 1: string
SELECT
    inventory.id AS inventory_id
FROM inventory
JOIN items ON items.inventory_id = inventory.id
WHERE inventory.id = ?
`
