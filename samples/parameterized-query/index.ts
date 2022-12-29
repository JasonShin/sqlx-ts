import { sql } from "sqlx-ts";

const sampleQuery = sql`
SELECT id, points
FROM items
WHERE items.points > ?
AND items.points < ?
AND items.points = ?;
`

const sampleQuery2 = sql`
SELECT id, points
FROM items
WHERE id IN (?);
`
