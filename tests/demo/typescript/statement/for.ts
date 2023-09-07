import { sql } from 'sqlx-ts'

for (;true;) {
    const for1 = sql`SELECT id FROM items`
}

const stuff = []

for (const item of stuff) {
    const for2 = sql`SELECT id FROM items`
}

for (const item in stuff) {
    const for3 = sql`SELECT id FROM items`
}