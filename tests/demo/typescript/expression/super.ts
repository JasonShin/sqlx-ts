import { sql } from 'sqlx-ts'

class SuperClass {
    constructor(sqlTest: string) {
        console.log(sqlTest);
    }
}

class ChildClass extends SuperClass {
    constructor() {
        super(sql`
        -- @name: super query
        SELECT id FROM items
        `);
    }
}

