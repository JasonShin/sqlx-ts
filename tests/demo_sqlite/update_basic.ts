import { sql } from 'sqlx-ts'

const updateItemName = sql`
-- @name: update item name
UPDATE items SET name = $1 WHERE id = $2
`

const updateCharacterLevel = sql`
-- @name: update character level
UPDATE characters SET level = $1, experience = $2 WHERE id = $3
`
