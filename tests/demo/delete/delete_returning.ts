import { sql } from 'sqlx-ts'

// Issue #226: DELETE with RETURNING clause should generate types
const deleteReturningAll = sql`
-- @name: delete returning all
DELETE FROM items
WHERE id = $1
RETURNING *
`

const deleteReturningSpecific = sql`
-- @name: delete returning specific
DELETE FROM items
WHERE id = $1
RETURNING id, name
`

const deleteReturningWithAlias = sql`
-- @name: delete returning with alias
DELETE FROM items
WHERE id = $1
RETURNING id AS deleted_id, name AS deleted_name
`

const deleteReturningExpression = sql`
-- @name: delete returning expression
DELETE FROM items
WHERE id = $1
RETURNING id, UPPER(name) AS upper_name
`
