import { sql } from 'sqlx-ts'

// Issue #265: Backtick-escaped table names should work
const backtickTableName = sql`
-- @db: db_mysql
-- @name: backtick table name
SELECT * FROM \`items\`
`

// Issue #265: Backtick-escaped qualified table names (db.table)
const backtickQualifiedName = sql`
-- @db: db_mysql
-- @name: backtick qualified name
SELECT * FROM \`sqlx-ts\`.\`items\`
`

// Issue #265: Backtick-escaped column names
const backtickColumnNames = sql`
-- @db: db_mysql
-- @name: backtick column names
SELECT \`id\`, \`name\` FROM \`items\` WHERE \`id\` = ?
`
