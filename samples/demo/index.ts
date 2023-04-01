import { sql } from "sqlx-ts";

/*
const someInputQuery = sql`
INSERT INTO items (id, food_type, time_takes_to_cook, table_id, points)
VALUES
(?, ?, 2, 1, 2),
(1, 'test', ?, ?, ?);
`
*/

/*
const somePostgresInputQuery = sql`
INSERT INTO items (id, food_type, time_takes_to_cook, table_id, points)
VALUES
($2, $1, 2, $3, 2),
($5, 'test', $4, $7, $6);
`
*/
/*
const someDeleteQuery = sql`
DELETE FROM items WHERE id = ?;
`*/

/*
const someQuery = sql`
SELECT *
FROM items
WHERE points > ?
AND points < ?
OR points = ?
`
*/

/*
const someDeleteQuery = sql`
DELETE FROM items WHERE id = ?
`
*/

/*
const someUpdateQuery = sql`
UPDATE items
SET food_type = ?
WHERE id = ?
`
*/

/*
const someUpdateQuery2 = sql`
UPDATE items
JOIN tables ON tables.id = items.table_id
SET items.food_type = ?
WHERE tables.id = ?
`
*/

const someUpdateQuery3 = sql`
UPDATE items
JOIN tables ON tables.id = items.table_id
SET
    items.food_type = ?,
    items.time_takes_to_cook = ?
WHERE tables.id = ?
`

/**
// FROM syntax below
UPDATE 
    sales.commissions
SET  
    sales.commissions.commission = 
        c.base_amount  * COALESCE(t.percentage,0.1)
FROM  
    sales.commissions c
    LEFT JOIN sales.targets t 
        ON c.target_id = t.target_id;
 */

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

/*
// Querying from an unknown table
const someQuery = sql`SELECT * FROM indexjs_unknown`;

// Inserting more values than expected
const insertQuery = sql`
INSERT INTO items (food_type, time_takes_to_cook, table_id, points)
VALUES ('steak', 1, 1, 1, 1);
`;

///////////////////
// If statements //
///////////////////
if (true) {
const query3 = sql`SELECT * FROM if_statement1;`;
}
*/

