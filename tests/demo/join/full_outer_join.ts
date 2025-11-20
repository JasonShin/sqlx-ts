import { sql } from 'sqlx-ts'

// Basic FULL OUTER JOIN (PostgreSQL only)
const basicFullOuterJoin = sql`
-- @name: basic full outer join
SELECT items.id, items.name, inventory.quantity
FROM items
FULL OUTER JOIN inventory ON items.inventory_id = inventory.id
`

// FULL OUTER JOIN with COALESCE
const fullOuterJoinWithCoalesce = sql`
-- @name: full outer join with coalesce
SELECT
  COALESCE(items.id, inventory.id) AS id,
  items.name,
  inventory.quantity
FROM items
FULL OUTER JOIN inventory ON items.inventory_id = inventory.id
`
