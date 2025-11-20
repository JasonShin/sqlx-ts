import { sql } from 'sqlx-ts'

// AND operator
const andOperator = sql`
-- @name: and operator
SELECT id, name, rarity
FROM items
WHERE id > 3 AND rarity = 'common'
`

// OR operator
const orOperator = sql`
-- @name: or operator
SELECT id, name, rarity
FROM items
WHERE id < 3 OR rarity = 'legendary'
`

// NOT operator
const notOperator = sql`
-- @name: not operator
SELECT id, name, rarity
FROM items
WHERE NOT (rarity = 'common')
`

// Combined boolean logic
const combinedBooleanLogic = sql`
-- @name: combined boolean logic
SELECT id, name, rarity
FROM items
WHERE (id > 5 AND rarity = 'common')
   OR (id < 3 AND rarity = 'rare')
`

// Complex nested conditions
const nestedConditions = sql`
-- @name: nested conditions
SELECT id, name, rarity
FROM items
WHERE (
  (id BETWEEN 1 AND 5 AND rarity IN ('common', 'rare'))
  OR
  (id > 10 AND rarity IS NOT NULL)
)
AND name IS NOT NULL
`

// Boolean expressions in SELECT
const booleanInSelect = sql`
-- @name: boolean in select
SELECT
  id,
  name,
  rarity,
  (id > 5 AND rarity = 'rare') AS is_high_id_rare,
  (rarity IN ('legendary', 'epic')) AS is_premium
FROM items
`
