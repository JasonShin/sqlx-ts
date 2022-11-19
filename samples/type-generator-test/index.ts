import { sql } from "sqlx-ts";

/*
const testQuery = sql`
-- @name: some query
-- @db: default
SELECT
    id,
    EXISTS(SELECT 1 FROM items WHERE points > 0) AS test_name
FROM items
WHERE id = ?
AND points > ?
AND items.points < ?;
`;
*/

const testQuery2 = sql`
-- @name: some query 2
-- @db: default
SELECT
    id,
    EXISTS(SELECT 1 FROM items WHERE points > 0) AS test_name
FROM items
WHERE id = ?
AND points > ?
AND points < ?;
`

/*
const testQuery2 = sql`
SELECT
    id,
    EXISTS(SELECT 1 FROM items WHERE points > 0) AS hmm
FROM items
WHERE id = $id;
`;

const testQueryInList = sql`
SELECT 1 IN (1, 2, true) AS test_test;
`;

const testQueryResultAnnotation = sql`
# @result id -> string | boolean
# @result food_type -> number
SELECT
    id,
    food_type
FROM items
`
 */

/*
const testQueryWithAliasAndJoin = sql`
SELECT
    items.id as idz,
    t.id,
    EXISTS(SELECT 1 FROM items WHERE points > 0) AS test_name
FROM items
JOIN tables t on items.table_id = t.id;
`
*/

/*
const testQueryWithAliasAndJoin = sql`
SELECT
    items.id as id_aliased,
    t.number as number_aliased
FROM items
JOIN tables t on items.table_id = t.id;
`
*/

/*
const testInsert = sql`
    INSERT INTO items (food_type, time_takes_to_cook, table_id, points) VALUES (?, ?, ?, ?);
`

const test2 = sql`
    INSERT INTO items (food_type, time_takes_to_cook, table_id, points) VALUES $$items(food_type, time_takes_to_cook, table_id, points);
`

const test3 = sql`
    UPDATE items
    SET points = $points
    WHERE id = $id;
`

const test4 = sql`
    INSERT INTO items (points)
    SELECT number FROM tables WHERE id = $id;
`

const test5 = sql`
    INSERT INTO items (id, points)
    VALUES(DEFAULT, $points)
    RETURNING id;
`
*/
