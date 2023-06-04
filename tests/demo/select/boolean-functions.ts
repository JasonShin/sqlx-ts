import { sql } from 'sqlx-ts'

// Exists
const exists = sql`
SELECT EXISTS(SELECT * FROM items WHERE id = $1) AS exists
FROM items
`

// Is True, Is False, Is Null, Is Not Null
const isTrue = sql`
SELECT ($1 IS TRUE) as is_true
FROM items
`

const isFalse = sql`
SELECT ($1 IS FALSE) as is_false
FROM items
`

const inList = sql`
SELECT 1 IN (1, 2, 3) as test
FROM items
`
