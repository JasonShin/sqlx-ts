import { sql } from 'sqlx-ts'

const wildcard = sql`
INSERT INTO items
VALUES (1, 'korean', 20, 1, 5)
RETURNING *;
`

const selector = sql`
INSERT INTO items
VALUES (1, 'korean', 20, 1, 5)
RETURNING id;
`

const alias = sql`
INSERT INTO items
VALUES (1, 'korean', 20, 1, 5)
RETURNING id as id1;
`
