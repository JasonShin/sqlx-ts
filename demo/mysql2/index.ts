import { sql } from 'sqlx-ts'
import * as mysql from 'mysql2/promise'
import { IGetItems2Result, TestInsertParams, TestUpdateParams, TestDeleteParams } from './index.queries'

type Rows<T> = Array<T & mysql.RowDataPacket>

(async () => {

    const connection = await mysql.createConnection({
        host: '127.0.0.1',
        user: 'root',
        port: 33306,
        database: 'sqlx-ts',
    });

    const [rows] = await connection.execute<Rows<IGetItems2Result>>(sql`
    -- @name: getItems2
    SELECT * FROM items
    `)
    
    for (const row of rows) {
        const { id, rarity, name } = row
        console.log(id, rarity, name)
    }

    await connection.execute(sql`
    -- @name: testInsert
    -- @db: db_mysql
    INSERT INTO items (id, name, rarity, flavor_text) VALUES (?, ?, 'test', 'test');
    `, [1, 'test'] as TestInsertParams[0])

    const rarityType = 'test'

    await connection.query(sql`
        -- @name: testUpdate
        -- @db: db_mysql
        UPDATE items SET rarity = ? WHERE id = 1;
    `, [rarityType] as TestUpdateParams)

    await connection.query(sql`
        -- @name: testDelete
        -- @db: db_mysql
        DELETE FROM items WHERE rarity = ?;
    `, [rarityType] as TestDeleteParams)

    connection.destroy()
})()
