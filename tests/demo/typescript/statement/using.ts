import { sql } from 'sqlx-ts'

const getResource = () => ({
    [Symbol.asyncDispose]: async () => {
        const testAsyncUsing = sql`
    SELECT * FROM items;
    `
    },
});

{
    await using resource = getResource();
}
