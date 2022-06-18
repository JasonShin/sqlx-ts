import { sql } from 'sqlx-ts'

const test_world = sql`
/**
 *  @name: some query
 *  @db: default
 **/
SELECT
    id,
    EXISTS(SELECT 1 FROM items WHERE points > 0) AS hmm
FROM items
WHERE id = $id;
`

/*
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
