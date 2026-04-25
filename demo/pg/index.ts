import { sql } from 'sqlx-ts'
import { Client } from 'pg'
import {
    TestInsertParams, ITestInsertResult,
    TestUpdateParams, ITestUpdateResult, 
    TestDeleteParams, ITestDeleteResult, IGetItemsResult,
 } from './index.queries'


const client = new Client({
    host: 'localhost',
    port: 54321,
    database: 'postgres',
    user: 'postgres',
    password: 'postgres',
});

(async () => {
    const someQuery = await client.query(sql`
        SELECT * FROM items;
    `)

    for (const row of someQuery.rows) {
        const { id, food_type, points } = row
        console.log(id, food_type, points)
    }

    await client.query<ITestInsertResult, TestInsertParams>(sql`
        -- @name: testInsert
        INSERT INTO items (id, name, rarity, flavor_text) VALUES ($1, $2, 'test', 'test');
    `, [1, "hello"])

    const rarityType = 'test'

    await client.query<ITestUpdateResult, TestUpdateParams>(sql`
        -- @name: testUpdate
        UPDATE items SET rarity = $1 WHERE id = (SELECT id FROM items WHERE rarity = 'test' LIMIT 1);
    `, [rarityType])

    await client.query<ITestDeleteResult, TestDeleteParams>(sql`
        -- @name: testDelete
        DELETE FROM items WHERE rarity = $1;
    `, [rarityType])

    await client.end()

    class TestQueryRepository {
        getItems() {
            return client.query<IGetItemsResult>(sql`
                -- @name: getItems
                SELECT inventory.id as inventoryId FROM items
                JOIN inventory ON items.inventory_id = inventory.id;
            `)
        }
    }

    new TestQueryRepository()
})();
