import { sql } from 'sqlx-ts'

// Issue #265: Double-quoted identifiers should work in PostgreSQL
const doubleQuoteTableName = sql`
-- @name: double quote table name
SELECT * FROM "items"
`

// Double-quoted qualified table names (schema.table)
const doubleQuoteQualifiedName = sql`
-- @name: double quote qualified name
SELECT * FROM "public"."items"
`

// Double-quoted column names with param
const doubleQuoteColumnNames = sql`
-- @name: double quote column names
SELECT "id", "name" FROM "items" WHERE "id" = $1
`
