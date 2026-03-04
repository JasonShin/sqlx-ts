import { sql } from 'sqlx-ts'

// JSON_OBJECT basic - build object from columns
const jsonObjectBasic = sql`
-- @db: db_mysql
-- @db: db_mysql
-- @name: json object basic
SELECT
  items.id AS id,
  JSON_OBJECT('id', items.id, 'name', items.name, 'rarity', items.rarity) AS item_json
FROM items
`

// JSON_ARRAYAGG for aggregation - aggregate rows into JSON array
const jsonArrayAggregation = sql`
-- @db: db_mysql
-- @db: db_mysql
-- @name: json array aggregation
SELECT
  items.rarity AS rarity,
  JSON_ARRAYAGG(JSON_OBJECT('id', items.id, 'name', items.name)) AS items
FROM items
GROUP BY items.rarity
`

// JSON operators in SELECT - extract values
const jsonOperatorsSelect = sql`
-- @db: db_mysql
-- @db: db_mysql
-- @name: json operators select
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  JSON_UNQUOTE(JSON_EXTRACT(
    JSON_OBJECT('id', json_test_data.id, 'name', json_test_data.name),
    '$.name'
  )) AS extracted_name
FROM json_test_data
WHERE json_test_data.name = 'user_profile'
`
