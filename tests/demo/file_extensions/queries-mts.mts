import { sql } from 'sqlx-ts'

// .mts file extension test (ES Module TypeScript)
const mtsQuery = sql`
-- @name: mts query
SELECT id, name FROM items WHERE rarity = $1
`

export const mtsFunction = () => {
  return sql`
-- @name: mts function query
SELECT id, name FROM items WHERE id = $1
  `
}
