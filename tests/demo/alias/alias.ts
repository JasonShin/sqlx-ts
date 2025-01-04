import { sql } from 'sqlx-ts'

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

const sql4 = sql`
SELECT
    COUNT(*) AS the_count
FROM items
`

const sql5 = sql`
SELECT AVG(quantity) AS the_avg
FROM inventory
`

const sql6 = sql`
SELECT LOWER(varchar1) as lower_varchar
FROM random
`

const sql7 = sql`
SELECT NOW() AS current_time
FROM items
`
