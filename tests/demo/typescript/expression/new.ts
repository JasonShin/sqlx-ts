import { sql } from 'sqlx-ts'

class NewClass {
    constructor(theQuery: string) {
        console.log(theQuery)
    }
}

const testInstance = new NewClass(sql`
-- @name: new class
SELECT id FROM items
`)
