import { sql } from 'sqlx-ts'

// DISTINCT basic
const distinctBasic = sql`
-- @name: distinct basic
SELECT DISTINCT rarity FROM items
`

// DISTINCT multiple columns
const distinctMultiple = sql`
-- @name: distinct multiple
SELECT DISTINCT rarity, name FROM items
`

// DISTINCT with WHERE
const distinctWithWhere = sql`
-- @name: distinct with where
SELECT DISTINCT rarity FROM items
WHERE name IS NOT NULL
`

// DISTINCT with parameters
const distinctWithParams = sql`
-- @name: distinct with params
SELECT DISTINCT rarity FROM items
WHERE id > $1
`

// DISTINCT ON (PostgreSQL specific)
const distinctOn = sql`
-- @name: distinct on
SELECT DISTINCT ON (rarity) id, rarity, name
FROM items
ORDER BY rarity, id DESC
`

// DISTINCT with ORDER BY
const distinctWithOrderBy = sql`
-- @name: distinct with order by
SELECT DISTINCT rarity FROM items
ORDER BY rarity ASC
`

// DISTINCT with aggregate
const distinctWithAggregate = sql`
-- @name: distinct with aggregate
SELECT COUNT(DISTINCT rarity) AS unique_rarities FROM items
`
