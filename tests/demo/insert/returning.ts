import { sql } from 'sqlx-ts'

const insertWildcard = sql`
INSERT INTO items
VALUES (1, 'korean', 20, 1, 5)
RETURNING *;
`

const insertSelector = sql`
INSERT INTO items
VALUES (1, 'korean', 20, 1, 5)
RETURNING id, food_type;
`

const insertAlias = sql`
INSERT INTO items
VALUES (1, 'korean', 20, 1, 5)
RETURNING id as id1, food_type AS foodType1;
`

const insertQuoted = sql`
INSERT INTO "items"
VALUES (1, 'korean', 20, 1, 5)
RETURNING "id" as id1, food_type AS "foodType1";
`


const insertParams = sql`
INSERT INTO "items" (id, food_type, time_takes_to_cook, table_id, points, description)
VALUES (1, $1, 20, 1, 5, $2)
RETURNING "id" as id1, food_type AS "foodType1";
`
