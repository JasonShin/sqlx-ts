import { sql } from 'sqlx-ts'


// JSON_ARRAYAGG for aggregation - aggregate rows into JSON array
const jsonArrayAggregation = sql`
-- @db: db_mysql
-- @name: json array aggregation
SELECT
  items.rarity AS rarity,
  JSON_ARRAYAGG(JSON_OBJECT('id', items.id, 'name', items.name)) AS items
FROM items
GROUP BY items.rarity
`

// JSON_OBJECT basic - build object from columns
const jsonObjectBasic = sql`
-- @db: db_mysql
-- @name: json object basic
SELECT
  items.id AS id,
  JSON_OBJECT('id', items.id, 'name', items.name, 'rarity', items.rarity) AS item_json
FROM items
`
