import { sql } from 'sqlx-ts'

// Simple CTE with explicit column selection
const simpleCte = sql`
-- @name: simple cte
WITH filtered_items AS (
  SELECT id, name FROM items
)
SELECT id, name FROM filtered_items
`

// CTE with window function (RANK) — original issue #104
const rankWithCte = sql`
-- @name: rank with cte
WITH ranked_items AS (
  SELECT
    id,
    name,
    rarity,
    RANK() OVER (PARTITION BY rarity ORDER BY id) AS rk
  FROM items
)
SELECT id, name, rk FROM ranked_items WHERE rk = 1
`

// Multiple CTEs
const multipleCtes = sql`
-- @name: multiple ctes
WITH
  popular AS (
    SELECT id, name FROM items WHERE id > 10
  ),
  rare AS (
    SELECT id, name FROM items WHERE rarity = 'rare'
  )
SELECT id, name FROM popular
`

// CTE with wildcard in outer query
const cteWithWildcard = sql`
-- @name: cte with wildcard
WITH base AS (
  SELECT id, name FROM items
)
SELECT * FROM base
`
