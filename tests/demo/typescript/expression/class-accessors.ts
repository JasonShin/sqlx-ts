import { sql } from 'sqlx-ts'

class SetterGetter {
    private _test: string

    public get test() {
        return sql`
            -- @name: getter query
            SELECT id FROM items;`
    }

    public set test(someSql: string) {
        const setterQuery = sql`SELECT id FROM items`
        this._test = someSql
    }

    defaultMethod() {
        const defaultMethodQuery = sql`SELECT id FROM items`
    }

    privateMethod() {
        const privateMethodQuery = sql`SELECT id FROM items`
    }

    publicMethod() {
        const publicMethodQuery = sql`SELECT id FROM items`
    }
}
