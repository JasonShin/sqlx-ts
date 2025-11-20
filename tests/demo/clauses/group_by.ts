import { sql } from 'sqlx-ts'

// GROUP BY single column
const groupBySingle = sql`
-- @name: group by single
SELECT rarity, COUNT(*) AS count
FROM items
GROUP BY rarity
`

// GROUP BY multiple columns
const groupByMultiple = sql`
-- @name: group by multiple
SELECT rarity, name, COUNT(*) AS count
FROM items
GROUP BY rarity, name
`

// GROUP BY with aggregate functions
const groupByWithAggregates = sql`
-- @name: group by with aggregates
SELECT
  rarity,
  COUNT(*) AS total_count,
  COUNT(inventory_id) AS with_inventory,
  MAX(id) AS max_id,
  MIN(id) AS min_id
FROM items
GROUP BY rarity
`

// GROUP BY with WHERE
const groupByWithWhere = sql`
-- @name: group by with where
SELECT rarity, COUNT(*) AS count
FROM items
WHERE name IS NOT NULL
GROUP BY rarity
`

// GROUP BY with parameters
const groupByWithParams = sql`
-- @name: group by with params
SELECT rarity, COUNT(*) AS count
FROM items
WHERE rarity = $1
GROUP BY rarity
`

// GROUP BY with JOIN
const groupByWithJoin = sql`
-- @name: group by with join
SELECT items.rarity, SUM(inventory.quantity) AS total_quantity
FROM items
LEFT JOIN inventory ON items.inventory_id = inventory.id
GROUP BY items.rarity
`
