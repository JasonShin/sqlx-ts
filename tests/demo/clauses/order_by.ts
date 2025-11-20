import { sql } from 'sqlx-ts'

// ORDER BY single column ASC
const orderByAsc = sql`
-- @name: order by asc
SELECT id, name FROM items
ORDER BY name ASC
`

// ORDER BY single column DESC
const orderByDesc = sql`
-- @name: order by desc
SELECT id, name FROM items
ORDER BY id DESC
`

// ORDER BY multiple columns
const orderByMultiple = sql`
-- @name: order by multiple
SELECT id, name, rarity FROM items
ORDER BY rarity ASC, name DESC
`

// ORDER BY with NULL handling
const orderByNullsFirst = sql`
-- @name: order by nulls first
SELECT id, name, rarity FROM items
ORDER BY rarity NULLS FIRST
`

const orderByNullsLast = sql`
-- @name: order by nulls last
SELECT id, name, rarity FROM items
ORDER BY rarity NULLS LAST
`

// ORDER BY with expression
const orderByExpression = sql`
-- @name: order by expression
SELECT id, name FROM items
ORDER BY LOWER(name) ASC
`

// ORDER BY with parameters in WHERE
const orderByWithParams = sql`
-- @name: order by with params
SELECT id, name, rarity FROM items
WHERE rarity = $1
ORDER BY name ASC
`
