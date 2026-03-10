import { sql } from 'sqlx-ts'

// Test nested field access with JSON_EXTRACT
const jsonNestedAccess = sql`
-- @db: db_mysql
-- @name: json nested access
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  data -> '$.address' AS address_json,
  data -> '$.address.city' AS city_json,
  JSON_UNQUOTE(data -> '$.address.city') AS city,
  JSON_UNQUOTE(data -> '$.address.zipCode') AS zip_code,
  JSON_UNQUOTE(JSON_EXTRACT(data, '$.address.street')) AS street
FROM json_test_data
WHERE json_test_data.name = 'user_with_address'
`

// Test array element access by index
const jsonArrayAccess = sql`
-- @db: db_mysql
-- @name: json array access
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  data -> '$.items' AS items_json,
  data -> '$.items[0]' AS first_item_json,
  data -> '$.items[1]' AS second_item_json,
  JSON_UNQUOTE(data -> '$.items[0].name') AS first_item_name,
  CAST(data -> '$.items[0].price' AS DECIMAL(10,2)) AS first_item_price
FROM json_test_data
WHERE json_test_data.name = 'shopping_cart'
`
