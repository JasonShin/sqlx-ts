import { sql } from 'sqlx-ts'

// SELECT query with a query name as an annotation
const testQuery = sql`
/**
 *  @name: some query
 *  @db: default
 **/
SELECT
    id,
    EXISTS(SELECT 1 FROM items WHERE points > 0) AS test_name
FROM items
WHERE id = $id;
`

// SELECT query without a query name as an annotation
const testQuery2 = sql`
SELECT
    id,
    EXISTS(SELECT 1 FROM items WHERE points > 0) AS hmm
FROM items
WHERE id = $id;
`

const testQueryInList = sql`
SELECT 1 IN (1, 2, true) AS test_test;
`

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
