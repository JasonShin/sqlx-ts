import { sql } from 'sqlx-ts'

const sql1 = sql`
-- @name: test ignore query
-- @db: db_mysql
SELECT *
FROM items;
`
