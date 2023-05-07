import { sql } from 'sqlx-ts'

/*
const sql1 = sql`
SELECT
    id as id1
FROM items;
`

const sql2 = sql`
SELECT items.id as id2
FROM items
`

const sql3 = sql`
SELECT items.id
FROM items
`
*/

const sql4 = sql`
SELECT
    COUNT(*) AS the_count
FROM items
`
