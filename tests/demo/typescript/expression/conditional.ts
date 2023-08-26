import { sql } from 'sqlx-ts'

// TODO: the following syntax is not working
// @ts-ignore
testData = true === 1 ? sql`
-- @name: truthy
SELECT id FROM items
` : sql`
-- @name: falsy
SELECT id FROM items
`
