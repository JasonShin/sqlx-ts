import { sql } from 'sqlx-ts'

// .cts file extension test (CommonJS TypeScript)
const ctsQuery = sql`
-- @name: cts query
SELECT id, name FROM items WHERE rarity = $1
`

module.exports = {
  ctsQuery,
  ctsFunction: () => {
    return sql`
-- @name: cts function query
SELECT id, name FROM items WHERE id = $1
    `
  }
}
