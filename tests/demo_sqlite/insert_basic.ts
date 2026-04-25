import { sql } from 'sqlx-ts'

const insertItem = sql`
-- @name: insert item
INSERT INTO items (name, rarity, flavor_text, inventory_id) VALUES ($1, $2, $3, $4)
`

const insertCharacter = sql`
-- @name: insert character
INSERT INTO characters (name, race_id, class_id, level) VALUES ($1, $2, $3, $4)
`
