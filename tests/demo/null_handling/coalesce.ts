import { sql } from 'sqlx-ts'

// COALESCE basic
const coalesceBasic = sql`
-- @name: coalesce basic
SELECT
  id,
  name,
  COALESCE(rarity, 'unknown') AS rarity_display
FROM items
`

// COALESCE with multiple fallbacks
const coalesceMultiple = sql`
-- @name: coalesce multiple
SELECT
  id,
  name,
  COALESCE(flavor_text, name, 'no description') AS description
FROM items
`

// COALESCE with numeric values
const coalesceNumeric = sql`
-- @name: coalesce numeric
SELECT
  id,
  name,
  COALESCE(inventory_id, 0) AS inventory_id_safe
FROM items
`

// COALESCE with aggregates
const coalesceWithAggregates = sql`
-- @name: coalesce with aggregates
SELECT
  COALESCE(rarity, 'unknown') AS rarity_group,
  COUNT(*) AS count
FROM items
GROUP BY 1
`
