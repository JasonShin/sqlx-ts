import { sql } from 'sqlx-ts'


// Test -> operator (get JSON field as JSON)
const jsonFieldAccess = sql`
-- @db: db_mysql
-- @name: json field access
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  data -> '$.username' AS username_json,
  data -> '$.age' AS age_json,
  data -> '$.active' AS active_json
FROM json_test_data
WHERE json_test_data.name = 'user_profile'
`

// Test ->> operator (get JSON field as text)
const jsonFieldAccessText = sql`
-- @db: db_mysql
-- @name: json field access text
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  data ->> '$.username' AS username,
  data ->> '$.email' AS email,
  CAST(data ->> '$.age' AS UNSIGNED) AS age,
  CAST(data ->> '$.active' AS UNSIGNED) AS active
FROM json_test_data
WHERE json_test_data.name = 'user_profile'
`

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

// Test path access with deep nesting
const jsonPathAccess = sql`
-- @db: db_mysql
-- @name: json path access
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  data -> '$.stats.level' AS level_json,
  data -> '$.stats.inventory[0].item' AS first_item_json,
  CAST(JSON_UNQUOTE(data -> '$.stats.level') AS UNSIGNED) AS level,
  JSON_UNQUOTE(data -> '$.stats.inventory[0].item') AS first_item_name,
  JSON_UNQUOTE(data -> '$.stats.inventory[0].rarity') AS first_item_rarity
FROM json_test_data
WHERE json_test_data.name = 'game_stats'
`

// Test deep nested path access
const jsonDeepPathAccess = sql`
-- @db: db_mysql
-- @name: json deep path access
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  data -> '$.app.settings.database.host' AS db_host_json,
  JSON_UNQUOTE(data -> '$.app.settings.database.host') AS db_host,
  CAST(JSON_UNQUOTE(data -> '$.app.settings.database.port') AS UNSIGNED) AS db_port,
  CAST(JSON_UNQUOTE(data -> '$.app.settings.features.darkMode') AS UNSIGNED) AS dark_mode,
  CAST(JSON_UNQUOTE(data -> '$.app.settings.features.notifications.email') AS UNSIGNED) AS email_notifications
FROM json_test_data
WHERE json_test_data.name = 'nested_config'
`

// Test filtering by JSON field
const jsonFilterByField = sql`
-- @db: db_mysql
-- @name: json filter by field
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  data ->> '$.username' AS username,
  data ->> '$.email' AS email
FROM json_test_data
WHERE data ->> '$.active' = 'true'
  AND CAST(data ->> '$.age' AS UNSIGNED) > 25
`

// Test null handling in JSON
const jsonNullHandling = sql`
-- @db: db_mysql
-- @name: json null handling
SELECT
  json_test_data.id AS id,
  JSON_UNQUOTE(data -> '$.reviews[0].comment') AS first_comment,
  JSON_UNQUOTE(data -> '$.reviews[1].comment') AS second_comment,
  JSON_UNQUOTE(data -> '$.reviews[2].comment') AS third_comment,
  JSON_UNQUOTE(data -> '$.reviews[0].reviewer') AS first_reviewer,
  JSON_UNQUOTE(data -> '$.reviews[2].reviewer') AS third_reviewer
FROM json_test_data
WHERE json_test_data.name = 'product_reviews'
`
