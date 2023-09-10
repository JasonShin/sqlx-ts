import { sql } from 'sqlx-ts'

const testData = true

const test1 = testData ?? sql`
-- @name: nullish coalescing
SELECT id FROM items
`

const test2 = testData || sql`
-- @name: pipePipe
SELECT id FROM items
`

const test3 = testData && sql`
-- @name: ampersand ampersand
SELECT id FROM items
`
