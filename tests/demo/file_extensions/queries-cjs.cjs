const { sql } = require('sqlx-ts')

// .cjs file extension test (CommonJS JavaScript)
const cjsQuery = sql`
-- @name: cjs query
SELECT id, name FROM items WHERE rarity = $1
`

module.exports = {
  cjsQuery,
  cjsFunction: () => {
    return sql`
-- @name: cjs function query
SELECT id, name FROM items WHERE id = $1
    `
  }
}
