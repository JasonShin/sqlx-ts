import { sql } from 'sqlx-ts'

// MySQL SET type - select single SET column
const setSelect1 = sql`
-- @db: db_mysql
SELECT
  set1
FROM
  random
`

// MySQL SET type - select multiple columns including SET
const setSelect2 = sql`
-- @db: db_mysql
SELECT
  intz,
  set1,
  varchar1
FROM
  random
`

// MySQL SET type - with WHERE clause
const setSelect3 = sql`
-- @db: db_mysql
SELECT
  set1
FROM
  random
WHERE
  intz = ?
`
