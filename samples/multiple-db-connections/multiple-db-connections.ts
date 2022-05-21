import { sql } from 'sqlx-ts'

const defaultDb = sql`
    /* db: default */
    SELECT * FROM items;
`
