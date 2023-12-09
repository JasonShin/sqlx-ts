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
        const privDecoConstructor = sql`SELECT id FROM items`
    }

    findAll() {
        const privDecoMethod = sql`SELECT id FROM items`
    }

    async findNothing() {
        const privDecoMethod = sql`SELECT id FROM items`
    }

    public findPublic() {
        const pubDecoMethod = sql`SELECT id FROM items`
    }

    protected findProtected() {
        const protDecoMethod = sql`SELECT id FROM items`
    }
}

export class DecoClass2 {
    constructor() {
        console.log('class test')
        const privDecoConstructor = sql`SELECT id FROM items`
    }

    findAll() {
        const privDecoMethod = sql`SELECT id FROM items`
    }

    async findNothing() {
        const privDecoMethod = sql`SELECT id FROM items`
    }

    public findPublic() {
        const pubDecoMethod = sql`SELECT id FROM items`
    }

    protected findProtected() {
        const protDecoMethod = sql`SELECT id FROM items`
    }
}


export default class DecoClassDefault {
    constructor() {
        console.log('class test')
        const privDecoConstructor = sql`SELECT id FROM items`
    }

    findAll() {
        const privDecoMethod = sql`SELECT id FROM items`
    }

    async findNothing() {
        const privDecoMethod = sql`SELECT id FROM items`
    }

    public findPublic() {
        const pubDecoMethod = sql`SELECT id FROM items`
    }

    protected findProtected() {
        const protDecoMethod = sql`SELECT id FROM items`
    }
}
