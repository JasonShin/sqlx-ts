import { sql } from 'sqlx-ts'

const obj = {
    a: sql`
  -- @name: anotherTestObjectQuery  
  SELECT * FROM items
  `,
    b: {
        c: sql`
    -- @name: nestedTestObjectQuery
    SELECT * FROM items
    `
    }
}
