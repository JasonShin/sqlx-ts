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
-- @result points: string
SELECT
    points
FROM items;
`

const annotationSql4 = sql`
-- @param 1: string
SELECT *
FROM items
WHERE points > $1
`

const annotationSql5 = sql`
-- @result table_id: boolean
-- @param 1: string
SELECT
    tables.id AS table_id
FROM tables
JOIN items ON items.table_id = tables.id
WHERE tables.id = $1
`

const annotationSql6 = sql`
-- @name: test mysql query with param overrides
-- @db: db_mysql
-- @param 1: string
SELECT
    tables.id AS table_id
FROM tables
JOIN items ON items.table_id = tables.id
WHERE tables.id = ?
`
