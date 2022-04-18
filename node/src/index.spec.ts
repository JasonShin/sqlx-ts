import { sql } from './index'

test('sql tagged template literal should return raw sql back', () => {
    const rawSql = sql`SELECT * FROM test;`
})