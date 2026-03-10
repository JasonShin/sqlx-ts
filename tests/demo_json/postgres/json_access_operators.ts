import { sql } from 'sqlx-ts'

// Test -> operator (get JSON object field as JSON)
const jsonFieldAccess = sql`
-- @name: json field access
SELECT
  id,
  name,
  data -> 'username' AS username_json,
  data -> 'age' AS age_json,
  data -> 'active' AS active_json
FROM json_test_data
WHERE name = 'user_profile'
`

// Test ->> operator (get JSON object field as text)
const jsonFieldAccessText = sql`
-- @name: json field access text
SELECT
  id,
  name,
  data ->> 'username' AS username,
  data ->> 'email' AS email,
  (data ->> 'age')::integer AS age,
  (data ->> 'active')::boolean AS active
FROM json_test_data
WHERE name = 'user_profile'
`

// Test nested field access with -> and ->>
const jsonNestedAccess = sql`
-- @name: json nested access
SELECT
  id,
  name,
  data -> 'address' AS address_json,
  data -> 'address' -> 'city' AS city_json,
  data -> 'address' ->> 'city' AS city,
  data -> 'address' ->> 'zipCode' AS zip_code,
  data #> '{address, street}' AS street_json,
  data #>> '{address, street}' AS street
FROM json_test_data
WHERE name = 'user_with_address'
`

// Test array element access by index
const jsonArrayAccess = sql`
-- @name: json array access
SELECT
  id,
  name,
  data -> 'items' AS items_json,
  data -> 'items' -> 0 AS first_item_json,
  data -> 'items' -> 1 AS second_item_json,
  data -> 'items' -> 0 ->> 'name' AS first_item_name,
  data -> 'items' -> 0 ->> 'price' AS first_item_price
FROM json_test_data
WHERE name = 'shopping_cart'
`

// Test #> operator (get JSON object at path)
const jsonPathAccess = sql`
-- @name: json path access
SELECT
  id,
  name,
  data #> '{stats, level}' AS level_json,
  data #> '{stats, inventory, 0, item}' AS first_item_json,
  data #>> '{stats, level}' AS level,
  data #>> '{stats, inventory, 0, item}' AS first_item_name,
  data #>> '{stats, inventory, 0, rarity}' AS first_item_rarity
FROM json_test_data
WHERE name = 'game_stats'
`

// Test deep nested path access
const jsonDeepPathAccess = sql`
-- @name: json deep path access
SELECT
  id,
  name,
  data #> '{app, settings, database, host}' AS db_host_json,
  data #>> '{app, settings, database, host}' AS db_host,
  data #>> '{app, settings, database, port}' AS db_port,
  data #>> '{app, settings, features, darkMode}' AS dark_mode,
  data #>> '{app, settings, features, notifications, email}' AS email_notifications
FROM json_test_data
WHERE name = 'nested_config'
`

// Test mixed operators in WHERE clause
const jsonFilterByField = sql`
-- @name: json filter by field
SELECT
  id,
  name,
  data ->> 'username' AS username,
  data ->> 'email' AS email
FROM json_test_data
WHERE data ->> 'active' = 'true'
  AND (data ->> 'age')::integer > 25
`

// Test null handling in JSON
const jsonNullHandling = sql`
-- @name: json null handling
SELECT
  id,
  data -> 'reviews' -> 0 ->> 'comment' AS first_comment,
  data -> 'reviews' -> 1 ->> 'comment' AS second_comment,
  data -> 'reviews' -> 2 ->> 'comment' AS third_comment,
  data -> 'reviews' -> 0 ->> 'reviewer' AS first_reviewer,
  data -> 'reviews' -> 2 ->> 'reviewer' AS third_reviewer
FROM json_test_data
WHERE name = 'product_reviews'
`
