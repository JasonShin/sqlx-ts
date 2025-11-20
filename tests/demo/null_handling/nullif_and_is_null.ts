import { sql } from 'sqlx-ts'

// IS NULL basic
const isNullBasic = sql`
-- @name: is null basic
SELECT id, name, rarity
FROM items
WHERE rarity IS NULL
`

// IS NOT NULL
const isNotNull = sql`
-- @name: is not null
SELECT id, name, rarity
FROM items
WHERE rarity IS NOT NULL
`

// NULLIF basic
const nullifBasic = sql`
-- @name: nullif basic
SELECT
  id,
  name,
  NULLIF(rarity, 'common') AS rarity_excluding_common
FROM items
`

// NULLIF with empty string
const nullifEmptyString = sql`
-- @name: nullif empty string
SELECT
  id,
  name,
  NULLIF(name, '') AS name_safe
FROM items
`

// IS DISTINCT FROM
const isDistinctFrom = sql`
-- @name: is distinct from
SELECT id, name, rarity
FROM items
WHERE rarity IS DISTINCT FROM $1
`

// IS NOT DISTINCT FROM
const isNotDistinctFrom = sql`
-- @name: is not distinct from
SELECT id, name, rarity
FROM items
WHERE rarity IS NOT DISTINCT FROM $1
`
