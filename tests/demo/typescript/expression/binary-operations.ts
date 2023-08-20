import { sql } from 'sqlx-ts'

let testData = ''

// logical
testData = true ?? sql`
-- @name: nullish coalescing
SELECT id FROM items
`

testData = true || sql`
-- @name: pipePipe
SELECT id FROM items
`

testData = true && sql`
-- @name: ampersand ampersand
SELECT id FROM items
`

testData = 1 < sql`
-- @name: less
SELECT id FROM items;
`

1 <= sql`
-- @name: less than
SELECT id FROM items;
`

1 > sql`
-- @name: greater than
SELECT id FROM items;
`

1 >= sql`
-- @name: greater equal
SELECT id FROM items;
`

1 == sql`
-- @name: equal equal
SELECT id FROM items;
`

1 === sql`
-- @name: equal equal equal
SELECT id FROM items;
`
