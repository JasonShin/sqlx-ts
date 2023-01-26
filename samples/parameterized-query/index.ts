import { sql } from "sqlx-ts";

const someQuery = sql`
SELECT *
FROM items
WHERE points > ?
AND points < ?
OR points = ?
`


/*
const subQuery1 = sql`
-- @db: default
SELECT id, points
FROM items
WHERE id IN (SELECT id FROM items WHERE points > $1);
`
*/

/*
const subQuery2 = sql`
-- @db: default
SELECT id, points
FROM items
WHERE id = (SELECT id FROM items WHERE id = $1);
`
*/

/*
const subquery3 = sql`
-- @db: default
SELECT
	id,
	(SELECT number FROM tables WHERE items.table_id = tables.id and tables.number > $1) as test
FROM items
`
*/
