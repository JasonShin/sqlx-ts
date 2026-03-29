import { sql } from 'sqlx-ts'

// Issue #266: Placeholder before comparison operator
// The parameter $1 appears on the LEFT side of >=
const placeholderBeforeComparison = sql`
-- @name: placeholder before comparison
SELECT *
FROM inventory
WHERE $1 >= quantity
AND quantity >= $2
`

// Issue #266: Placeholder as BETWEEN expr
// The parameter $1 is the value being tested, columns are the bounds
const placeholderBetweenExpr = sql`
-- @name: placeholder between expr
SELECT *
FROM inventory
WHERE $1 BETWEEN quantity AND quantity
`

// Issue #266: Standard BETWEEN with placeholder bounds
// The column is the value, parameters $1 and $2 are the bounds
const betweenPlaceholderBounds = sql`
-- @name: between placeholder bounds
SELECT *
FROM inventory
WHERE quantity BETWEEN $1 AND $2
`

// Mixed: normal right-side param + left-side param in same query
const mixedParamPositions = sql`
-- @name: mixed param positions
SELECT *
FROM inventory
WHERE quantity = $1
AND $2 >= quantity
`
