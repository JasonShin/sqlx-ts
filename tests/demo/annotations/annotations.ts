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
-- @results points: string
SELECT
    points
FROM items;
`
