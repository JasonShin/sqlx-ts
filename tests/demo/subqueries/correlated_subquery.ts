import { sql } from 'sqlx-ts'

// Correlated subquery with EXISTS
const correlatedWithExists = sql`
-- @name: correlated with exists
SELECT id, name, rarity
FROM items i1
WHERE EXISTS (
  SELECT 1 FROM items i2
  WHERE i2.rarity = i1.rarity AND i2.id != i1.id
)
`

// Correlated subquery with NOT EXISTS
const correlatedWithNotExists = sql`
-- @name: correlated with not exists
SELECT id, name, rarity
FROM items i1
WHERE NOT EXISTS (
  SELECT 1 FROM items i2
  WHERE i2.rarity = i1.rarity AND i2.id > i1.id
)
`

// Correlated subquery in WHERE
const correlatedInWhere = sql`
-- @name: correlated in where
SELECT id, name, rarity
FROM items i1
WHERE id > (
  SELECT AVG(id) FROM items i2 WHERE i2.rarity = i1.rarity
)
`

// Correlated subquery with parameter
const correlatedWithParam = sql`
-- @name: correlated with param
SELECT id, name, rarity
FROM items i1
WHERE EXISTS (
  SELECT 1 FROM inventory
  WHERE inventory.id = i1.inventory_id AND inventory.quantity > $1
)
`
