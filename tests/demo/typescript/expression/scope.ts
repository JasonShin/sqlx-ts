import { sql } from 'sqlx-ts'

(
    sql`
  -- @name: testParenthesisQuery
  SELECT * FROM items
  `
)
