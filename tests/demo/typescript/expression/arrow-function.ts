import { sql } from 'sqlx-ts'

const plainArrowFunction = () => sql`SELECT id FROM items`

const nameOverrideArrowFunction = () => sql`
-- @name: name overridden arrow func
SELECT id FROM items;
`

const arrowFunction = () => {
    return sql`
-- @name: arrow function
SELECT id FROM items
`
}

const arrowFunctionDefaultArgument = (x = sql`SELECT * FROM items`) => null
