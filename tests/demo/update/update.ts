import { sql } from 'sqlx-ts'

const updateQuery = sql`
UPDATE items
SET name = $1, rarity = $2
WHERE id = $3
`

const quotedUpdateQuery = sql`
UPDATE "items"
SET "name" = $1, "rarity" = $2
WHERE "id" = $3
`

const nullableFieldUpdate = sql`
UPDATE "items"
SET rarity = $1
WHERE "id" = $2
`
