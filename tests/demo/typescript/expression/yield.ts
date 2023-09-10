import { sql } from 'sqlx-ts'

function *gen() {
    yield sql`
    -- @name: yield query
    SELECT id FROM items;
    `
}