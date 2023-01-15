import { sql } from "sqlx-ts";

const subQuery1 = sql`
-- @db: default
SELECT id, points
FROM items
WHERE id IN (SELECT id FROM items WHERE points > $1);
`

const subQuery2 = sql`
-- @db: default
SELECT id, points
FROM items
WHERE id = (SELECT id FROM items WHERE id = $1)
`
