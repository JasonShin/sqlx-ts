import { sql } from './index'

test('should return single line', () => {
    const rawSql = sql`SELECT * FROM test;`

    expect(rawSql).toBe('SELECT * FROM test;')
})

test('should return multiple lines', () => {
    const rawSql = sql`
        SELECT *
        FROM test
        WHERE createdAt > 2019-1-1;
    `

    expect(rawSql).toBe(`
        SELECT *
        FROM test
        WHERE createdAt > 2019-1-1;
    `)
})

test('should return empty', () => {
    const rawSql = sql``

    expect(rawSql).toBe('')
})
