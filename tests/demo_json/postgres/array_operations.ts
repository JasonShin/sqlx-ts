import { sql } from 'sqlx-ts'

// array_agg basic
const arrayAggBasic = sql`
-- @name: array agg basic
SELECT
  rarity,
  array_agg(name) AS names
FROM items
GROUP BY rarity
`

// array_agg with ORDER BY
const arrayAggWithOrderBy = sql`
-- @name: array agg with order by
SELECT
  rarity,
  array_agg(name ORDER BY id) AS names_ordered
FROM items
GROUP BY rarity
`

// Array literal and unknown
const arrayLiteralunknown = sql`
-- @name: array literal unknown
SELECT id, name, rarity
FROM items
WHERE rarity = ANY(ARRAY['common', 'rare'])
`

// Array with parameters
const arrayWithParams = sql`
-- @name: array with params
SELECT id, name, rarity
FROM items
WHERE rarity = ANY($1::text[])
`
