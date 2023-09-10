import { sql } from 'sqlx-ts'

function test() {
    const functionAssign = sql`SELECT id FROM items`

    return sql`
    -- @name: return query
    SELECT id FROM items
    `;
}
