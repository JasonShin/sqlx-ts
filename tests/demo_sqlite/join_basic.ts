import { sql } from 'sqlx-ts'

const selectItemsWithInventory = sql`
-- @name: select items with inventory
SELECT items.id, items.name, items.rarity, inventory.quantity
FROM items
JOIN inventory ON items.inventory_id = inventory.id
WHERE inventory.quantity > $1
`
