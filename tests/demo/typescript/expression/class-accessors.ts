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

class TestClass {
    private sql1 = sql`
  -- @name: testClassPropertyQuery
    SELECT id FROM items
  `
    constructor(z: string) {
        const query = sql`
    -- @name: testClassConstructorQuery
    SELECT id FROM items
    `
    }

    someMethod() {
        const query = sql`
    -- @name: testClassMethodQuery
    SELECT id FROM items
    `
    }
}

class ChildClass extends TestClass {
    constructor() {
        super(sql`
      -- @name: testChildClassConstructorQuery
      SELECT id FROM items
    `)
    }

    hmm() {
        let z = this.someMethod
    }
}

// AutoAccessor
class AutoAccessorTest {
    private accessor privAutoAccessorProp: string = sql`
    SELECT * FROM items;
    `
    accessor autoAccessorProp: string = sql`
    SELECT * FROM items;
    `
}
