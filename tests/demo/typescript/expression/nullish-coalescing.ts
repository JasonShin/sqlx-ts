import { sql } from 'sqlx-ts'

const something = false
const somethingElse = something ?? sql`
-- @name: testNullishCoalescingQuery
SELECT * FROM items
`
