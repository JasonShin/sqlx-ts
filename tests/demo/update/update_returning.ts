import { sql } from 'sqlx-ts'

// Issue #226: UPDATE with RETURNING clause should generate types
const updateReturningAll = sql`
-- @name: update returning all
UPDATE items
SET name = $1
WHERE id = $2
RETURNING *
`

const updateReturningSpecific = sql`
-- @name: update returning specific
UPDATE items
SET name = $1
WHERE id = $2
RETURNING id, name
`

const updateReturningWithAlias = sql`
-- @name: update returning with alias
UPDATE items
SET name = $1
WHERE id = $2
RETURNING id AS updated_id, name AS updated_name
`

const updateReturningExpression = sql`
-- @name: update returning expression
UPDATE items
SET name = $1
WHERE id = $2
RETURNING id, LOWER(name) AS lower_name
`
