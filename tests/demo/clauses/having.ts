import { sql } from 'sqlx-ts'

// HAVING basic
const havingBasic = sql`
-- @name: having basic
SELECT rarity, COUNT(*) AS count
FROM items
GROUP BY rarity
HAVING COUNT(*) > 1
`

// HAVING with aggregate condition
const havingWithAggregate = sql`
-- @name: having with aggregate
SELECT rarity, COUNT(*) AS count, MAX(id) AS max_id
FROM items
GROUP BY rarity
HAVING COUNT(*) > $1 AND MAX(id) > $2
`

// HAVING with SUM
const havingWithSum = sql`
-- @name: having with sum
SELECT items.rarity, SUM(inventory.quantity) AS total_quantity
FROM items
LEFT JOIN inventory ON items.inventory_id = inventory.id
GROUP BY items.rarity
HAVING SUM(inventory.quantity) > 10
`

// HAVING with multiple conditions
const havingMultipleConditions = sql`
-- @name: having multiple conditions
SELECT rarity, COUNT(*) AS count, AVG(id) AS avg_id
FROM items
GROUP BY rarity
HAVING COUNT(*) > 1 AND AVG(id) < 100
`
