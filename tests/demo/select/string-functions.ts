import { sql } from 'sqlx-ts'

const someUpdateQuery = sql`
SELECT OVERLAY('DONALD DUCK' PLACING 'TRUMP' FROM 8) AS Z
FROM items;
`
