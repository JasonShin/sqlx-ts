import { sql } from 'sqlx-ts'

const updateQuery = sql`
UPDATE items
SET food_type = $1, time_takes_to_cook = $2
WHERE id = $3
`

const quotedUpdateQuery = sql`
UPDATE "items"
SET "food_type" = $1, "time_takes_to_cook" = $2
WHERE "id" = $3
`
