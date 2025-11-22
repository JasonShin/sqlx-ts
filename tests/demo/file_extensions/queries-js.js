const { sql } = require('sqlx-ts')

// .js file extension test (JavaScript)
const jsQuery = sql`
-- @name: js query
SELECT id, name FROM items WHERE rarity = $1
`

const jsFunction = () => {
  return sql`
-- @name: js function query
SELECT id, name FROM items WHERE id = $1
  `
}

module.exports = {
  jsQuery,
  jsFunction
}
