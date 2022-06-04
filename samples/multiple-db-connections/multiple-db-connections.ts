import { sql } from 'sqlx-ts'

const defaultDb = sql`
    /* db: default */
    SELECT * FROM items;
`

const postgresDb = sql`
    /* db: db_mysql */
    SELECT * FROM itemz;
`
