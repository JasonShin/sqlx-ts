import { sql } from 'sqlx-ts'

const assignment = sql`
SELECT id FROM items;
`

let testString = 'test'

testString += sql`
-- @name: plus equal query
SELECT id FROM items;
`

testString -= sql`
-- @name: minus equal query
SELECT id FROM items;
`

testString *= sql`
-- @name: multiply equal query
SELECT id FROM items;
`

testString **= sql`
-- @name: mul-mul equal query
SELECT id FROM items;
`

testString /= sql`
-- @name: slash equal query
SELECT id FROM items;
`

testString %= sql`
-- @name: percent equal query
SELECT id FROM items;
`

testString <<= sql`
-- @name: less less equal query
SELECT id FROM items;
`

testString >>= sql`
-- @name: greater greater equal query
SELECT id FROM items;
`

testString >>>= sql`
-- @name: greater greater greater equal query
SELECT id FROM items;
`

testString &= sql`
-- @name: ampersand equal query
SELECT id FROM items;
`

testString &= sql`
-- @name: ampersand ampersand equal query
SELECT id FROM items;
`

testString |= sql`
-- @name: bar equal query
SELECT id FROM items;
`

testString ||= sql`
-- @name: bar bar equal query
SELECT id FROM items;
`

testString ??= sql`
-- @name: question question equal query
SELECT id FROM items;
`

testString ^= sql`
-- @name: caret equal query
SELECT id FROM items;
`
