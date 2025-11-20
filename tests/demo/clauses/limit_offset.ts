import { sql } from 'sqlx-ts'

// LIMIT basic
const limitBasic = sql`
-- @name: limit basic
SELECT id, name FROM items
LIMIT 10
`

// OFFSET basic
const offsetBasic = sql`
-- @name: offset basic
SELECT id, name FROM items
OFFSET 5
`

// LIMIT with OFFSET (pagination)
const limitOffset = sql`
-- @name: limit offset
SELECT id, name FROM items
LIMIT 10 OFFSET 20
`

// LIMIT with ORDER BY
const limitWithOrderBy = sql`
-- @name: limit with order by
SELECT id, name FROM items
ORDER BY id DESC
LIMIT 5
`

// Pagination with parameters
const paginationWithParams = sql`
-- @name: pagination with params
SELECT id, name, rarity FROM items
WHERE rarity = $1
ORDER BY id ASC
LIMIT $2 OFFSET $3
`

// FETCH FIRST (alternative syntax)
const fetchFirst = sql`
-- @name: fetch first
SELECT id, name FROM items
ORDER BY id
FETCH FIRST 10 ROWS ONLY
`

// FETCH with OFFSET
const fetchWithOffset = sql`
-- @name: fetch with offset
SELECT id, name FROM items
ORDER BY id
OFFSET 5 ROWS
FETCH NEXT 10 ROWS ONLY
`
