import { sql } from 'sqlx-ts'

// MySQL SET type - insert single value
const setInsert1 = sql`
-- @db: db_mysql
INSERT INTO
  random (set1)
VALUES
  (?)
`

// MySQL SET type - insert multiple columns including SET
const setInsert2 = sql`
-- @db: db_mysql
INSERT INTO
  random (intz, set1, varchar1)
VALUES
  (?, ?, ?)
`

// MySQL SET type - insert with multiple rows
const setInsert3 = sql`
-- @db: db_mysql
INSERT INTO
  random (set1)
VALUES
  (?),
  (?)
`
