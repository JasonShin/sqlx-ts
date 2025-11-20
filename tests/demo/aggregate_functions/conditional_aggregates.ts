import { sql } from 'sqlx-ts'

// COUNT with FILTER
const countWithFilter = sql`
-- @name: count with filter
SELECT
  rarity,
  COUNT(*) FILTER (WHERE id > 5) AS count_above_5,
  COUNT(*) FILTER (WHERE id <= 5) AS count_at_or_below_5
FROM items
GROUP BY rarity
`

// SUM with FILTER
const sumWithFilter = sql`
-- @name: sum with filter
SELECT
  rarity,
  SUM(id) FILTER (WHERE id > 5) AS sum_above_5,
  SUM(id) FILTER (WHERE id <= 5) AS sum_at_or_below_5
FROM items
GROUP BY rarity
`

// AVG with FILTER
const avgWithFilter = sql`
-- @name: avg with filter
SELECT
  rarity,
  AVG(id) FILTER (WHERE id > 5) AS avg_above_5
FROM items
GROUP BY rarity
`

// Multiple FILTER conditions
const multipleFilters = sql`
-- @name: multiple filters
SELECT
  COUNT(*) FILTER (WHERE rarity = 'common') AS count_common,
  COUNT(*) FILTER (WHERE rarity = 'rare') AS count_rare,
  COUNT(*) FILTER (WHERE rarity = 'legendary') AS count_legendary,
  COUNT(*) FILTER (WHERE rarity IS NULL) AS count_unknown
FROM items
`
