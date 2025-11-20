import { sql } from 'sqlx-ts'

// Basic LEFT JOIN
const basicLeftJoin = sql`
-- @name: basic left join
SELECT items.id, items.name, inventory.quantity
FROM items
LEFT JOIN inventory ON items.inventory_id = inventory.id
`

// LEFT JOIN with WHERE clause
const leftJoinWithWhere = sql`
-- @name: left join with where
SELECT items.id, items.name, inventory.quantity
FROM items
LEFT JOIN inventory ON items.inventory_id = inventory.id
WHERE items.rarity = $1
`

// LEFT JOIN with NULL handling
const leftJoinNullHandling = sql`
-- @name: left join null handling
SELECT items.id, items.name, COALESCE(inventory.quantity, 0) AS quantity
FROM items
LEFT JOIN inventory ON items.inventory_id = inventory.id
`

// Multiple LEFT JOINs
const multipleLeftJoins = sql`
-- @name: multiple left joins
SELECT items.id, items.name, inventory.quantity
FROM items
LEFT JOIN inventory ON items.inventory_id = inventory.id
LEFT JOIN characters ON inventory.character_id = characters.id
`
