import { sql } from 'sqlx-ts'

// Basic CROSS JOIN
const basicCrossJoin = sql`
-- @name: basic cross join
SELECT items.id, items.name, inventory.quantity
FROM items
CROSS JOIN inventory
`

// CROSS JOIN with WHERE filter
const crossJoinWithWhere = sql`
-- @name: cross join with where
SELECT items.id, items.name, inventory.quantity
FROM items
CROSS JOIN inventory
WHERE items.rarity = $1
`
