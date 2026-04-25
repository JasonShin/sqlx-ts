import { sql } from 'sqlx-ts'

const deleteItem = sql`
-- @name: delete item
DELETE FROM items WHERE id = $1
`

const deleteCharacter = sql`
-- @name: delete character
DELETE FROM characters WHERE id = $1
`
