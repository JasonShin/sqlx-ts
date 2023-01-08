import { sql } from "sqlx-ts";
/*
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
*/

const subQuery1 = sql`
SELECT id, points
FROM items
WHERE id IN (SELECT id FROM items WHERE points > ?);
`

const subQuery2 = sql`
SELECT id, points
FROM items
WHERE id = (SELECT id FROM items WHERE id = ?)
`
