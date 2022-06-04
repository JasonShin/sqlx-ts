import { sql } from 'sqlx-ts'

// It should always fallback to  default
const nonameDB = sql`SELECT * FROM items;`

const defaultDb = sql`
    /* db: default */
    SELECT * FROM items;
`

const postgresDb = sql`
    /* db: db_mysql */
    SELECT * FROM itemz;
`
