import { sql } from 'sqlx-ts'

// LIKE operator basic
const likeBasic = sql`
-- @name: like basic
SELECT id, name, rarity
FROM items
WHERE name LIKE 'S%'
`

// LIKE with parameter
const likeWithParam = sql`
-- @name: like with param
SELECT id, name, rarity
FROM items
WHERE name LIKE $1
`

// ILIKE (case-insensitive LIKE)
const ilike = sql`
-- @name: ilike
SELECT id, name, rarity
FROM items
WHERE name ILIKE '%sword%'
`

// NOT LIKE
const notLike = sql`
-- @name: not like
SELECT id, name, rarity
FROM items
WHERE name NOT LIKE 'A%'
`

// SIMILAR TO (regex pattern)
const similarTo = sql`
-- @name: similar to
SELECT id, name, rarity
FROM items
WHERE name SIMILAR TO '%(sword|shield)%'
`
