import { sql } from 'sqlx-ts'

const enumQuery = sql`
-- @db: db_mysql
SELECT enum1
FROM random
`
