import { sql } from 'sqlx-ts'

const arraySql = [sql`
-- @name: arrayQuery  
SELECT id FROM items
`]
