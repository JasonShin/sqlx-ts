import { sql } from 'sqlx-ts'

const sql1 = sql`
SELECT
    id as id1
FROM items;
`
