import { sql } from 'sqlx-ts'

with ([1, 2, 3]) {
    const withSql = `SELECT id FROM items WHERE $1;`
}
