import { sql } from 'sqlx-ts'

function SomeDeco(target: typeof DecoClass, context): typeof DecoClass {
    console.log('target', target)
    console.log('context', context)
    return target
}

@SomeDeco
class DecoClass {
    constructor() {
        console.log('class test')
        const decoQuery = sql`SELECT id FROM items`
    }
}
