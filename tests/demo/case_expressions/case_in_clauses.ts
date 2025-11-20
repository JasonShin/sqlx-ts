import { sql } from 'sqlx-ts'

// CASE in WHERE clause
const caseInWhere = sql`
-- @name: case in where
SELECT id, name, rarity
FROM items
WHERE CASE
  WHEN rarity = 'legendary' THEN id > 5
  WHEN rarity = 'rare' THEN id > 3
  ELSE id > 1
END
`

// CASE in ORDER BY clause
const caseInOrderBy = sql`
-- @name: case in order by
SELECT id, name, rarity
FROM items
ORDER BY CASE rarity
  WHEN 'legendary' THEN 1
  WHEN 'rare' THEN 2
  WHEN 'common' THEN 3
  ELSE 4
END, name
`

// CASE in GROUP BY and HAVING
const caseInGroupByHaving = sql`
-- @name: case in group by having
SELECT
  CASE
    WHEN rarity = 'legendary' THEN 'high'
    WHEN rarity = 'rare' THEN 'medium'
    ELSE 'low'
  END AS rarity_group,
  COUNT(*) AS count
FROM items
GROUP BY CASE
  WHEN rarity = 'legendary' THEN 'high'
  WHEN rarity = 'rare' THEN 'medium'
  ELSE 'low'
END
HAVING COUNT(*) > 0
`

// CASE with aggregates
const caseWithAggregates = sql`
-- @name: case with aggregates
SELECT
  rarity,
  SUM(CASE WHEN id > 5 THEN 1 ELSE 0 END) AS count_above_5,
  SUM(CASE WHEN id <= 5 THEN 1 ELSE 0 END) AS count_at_or_below_5
FROM items
GROUP BY rarity
`
