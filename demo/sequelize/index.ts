import {sql} from 'sqlx-ts'
import {QueryTypes, Sequelize} from 'sequelize'
import {
    ISomeQueryResult,
    TestInsertParams,
    TestUpdateParams,
    TestDeleteParams,
 } from "./index.queries";


const sequelize = new Sequelize('postgres://postgres:postgres@127.0.0.1:54321', {
    dialect: 'postgres'
})

async function demo() {
    const someQuery = await sequelize.query<ISomeQueryResult>(sql`
        SELECT * FROM items;
    `, {
        type: QueryTypes.SELECT,
        replacements: [],
    })

    for (const row of someQuery) {
        const { id, rarity, name } = row
        console.log(id, rarity, name)
    }

    await sequelize.query(sql`
        -- @name: testInsert
        INSERT INTO items (id, name, rarity, flavor_text) VALUES ($1, $2, 'test', 'test');
    `, {
        type: QueryTypes.INSERT,
        // Unfortunately sequelize query does not allow you to type binding params for INSERT
        bind: [1, 'test'] as TestInsertParams,
    })

    const rarityType = 'rare'

    await sequelize.query(sql`
        -- @name: testUpdate
        UPDATE items SET rarity = $1 WHERE id = (SELECT id FROM items WHERE rarity = 'test' LIMIT 1);
    `, {
        type: QueryTypes.UPDATE,
        // Unfortunately sequelize query does not allow you to type binding params for UPDATE
        bind: [rarityType] as TestUpdateParams,
    })
    
    await sequelize.query(sql`
        -- @name: testDelete
        DELETE FROM items WHERE rarity = $1;
    `, {
        type: QueryTypes.DELETE,
        bind: [rarityType] as TestDeleteParams,
    })
}

(async () => {
    await demo()
    await sequelize.close()
})();
