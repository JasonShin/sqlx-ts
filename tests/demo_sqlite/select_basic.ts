import { sql } from 'sqlx-ts'

const selectAllItems = sql`
-- @name: select all items
SELECT * FROM items
`

const selectItemById = sql`
-- @name: select item by id
SELECT * FROM items WHERE id = $1
`

const selectItemsByName = sql`
-- @name: select items by name
SELECT id, name FROM items WHERE name = $1
`
