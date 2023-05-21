import { sql } from 'sqlx-ts'

const sql1 = sql`
-- @name: test mysql query
-- @db: db_mysql
SELECT *
FROM items;
`

const sql2 = sql`
-- @name: test postgres query
-- @db: default
SELECT *
FROM items;
`

const sql3 = sql`
-- @result points: string
SELECT
    points
FROM items;
`

const sql4 = sql`
-- @param 1: string
SELECT *
FROM items
WHERE points > $1
`

const sql5 = sql`
-- @result table_id: boolean
-- @param 1: string
SELECT
    tables.id AS table_id
FROM tablesz
JOIN items ON items.table_id = tables.id
WHERE tables.id = $1
`
