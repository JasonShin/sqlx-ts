import { sql } from 'sqlx-ts'

const weirdName = sql`SELECT id FROM items`
