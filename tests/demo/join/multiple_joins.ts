import { sql } from 'sqlx-ts'

// Mixed JOIN types
const mixedJoinTypes = sql`
-- @name: mixed join types
SELECT items.id, items.name
FROM items
INNER JOIN inventory ON items.inventory_id = inventory.id
LEFT JOIN inventory AS backup ON items.inventory_id = backup.id AND backup.quantity > 0
`

// Self JOIN
const selfJoin = sql`
-- @name: self join
SELECT i1.id, i1.name, i2.name AS related_name
FROM items i1
INNER JOIN items i2 ON i1.inventory_id = i2.inventory_id
WHERE i1.id <> i2.id
`

// Three-way JOIN
const threeWayJoin = sql`
-- @name: three way join
SELECT items.id, items.name, inventory.quantity
FROM items
INNER JOIN inventory ON items.inventory_id = inventory.id
INNER JOIN characters ON inventory.character_id = characters.id
WHERE characters.level > $1
`

// Complex JOIN with parameters
const complexJoinWithParams = sql`
-- @name: complex join with params
SELECT items.id, items.name, inventory.quantity
FROM items
LEFT JOIN inventory ON items.inventory_id = inventory.id
WHERE items.rarity = $1 AND (inventory.quantity IS NULL OR inventory.quantity > $2)
`
