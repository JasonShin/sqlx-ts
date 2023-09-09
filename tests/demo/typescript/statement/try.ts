import { sql } from 'sqlx-ts'

try {
    // const try1 = sql`SELECT id FROM items`
    const try2 = sql`SELECT 1 as some_number FROM items`
} catch {
    throw new Error('This should never happen')
}