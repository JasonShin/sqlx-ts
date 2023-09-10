import { sql } from 'sqlx-ts'
import { QueryTypes, Sequelize } from 'sequelize'

///// Sequelize /////

async function demo() {
  const sequelize = new Sequelize('postgres://')
  const result = await sequelize.query(sql`
    -- @name: testSequelizeQuery
    SELECT * FROM items
    WHERE id = $1;
  `!, {
    type: QueryTypes.SELECT,
    replacements: [],
  })
}

// Native driver
(async () => {
  await sql`
    -- @name: testAwaitQuery
    SELECT * FROM items
  `
  await sql`
    -- @name: testAwaitQuery2
    SELECT * FROM items
  `

  const awaitClientQuery = await client.query(sql`
      SELECT * FROM items;
  `)

  const [rows, i] = await connection.execute<Rows<IGetItems2Result>>(sql`
    -- @name: getItemsWithRows
    SELECT * FROM items
  `)

  await connection.execute(sql`
  -- @name: testInsert
  -- @db: db_mysql
  INSERT INTO items (food_type, points, time_takes_to_cook, table_id) VALUES (?, ?, 1, 1);
  `)

  connection.destroy()
})();
