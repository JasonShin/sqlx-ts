import { sql } from 'sqlx-ts'

// Basic RIGHT JOIN
const basicRightJoin = sql`
-- @name: basic right join
SELECT items.id, items.name, inventory.quantity
FROM items
RIGHT JOIN inventory ON items.inventory_id = inventory.id
`

// RIGHT JOIN with parameters
const rightJoinWithParams = sql`
-- @name: right join with params
SELECT items.id, items.name, inventory.quantity
FROM items
RIGHT JOIN inventory ON items.inventory_id = inventory.id
WHERE inventory.quantity > $1
`
