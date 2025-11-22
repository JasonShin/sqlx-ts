import { sql } from 'sqlx-ts'

// .mjs file extension test (ES Module JavaScript)
const mjsQuery = sql`
-- @name: mjs query
SELECT id, name FROM items WHERE rarity = $1
`

export const mjsFunction = () => {
  return sql`
-- @name: mjs function query
SELECT id, name FROM items WHERE id = $1
  `
}
