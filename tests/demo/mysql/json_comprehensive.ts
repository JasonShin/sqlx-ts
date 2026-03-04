import { sql } from 'sqlx-ts'


// Test 1: JSON_EXTRACT with -> operator
const jsonExtract = sql`
-- @db: db_mysql
-- @name: json extract
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  JSON_UNQUOTE(JSON_EXTRACT(data, '$.username')) AS username,
  JSON_UNQUOTE(JSON_EXTRACT(data, '$.email')) AS email,
  CAST(JSON_EXTRACT(data, '$.age') AS UNSIGNED) AS age
FROM json_test_data
WHERE json_test_data.name = 'user_profile'
`

// Test 2: JSON_EXTRACT with ->> operator (shorthand)
const jsonExtractShorthand = sql`
-- @db: db_mysql
-- @name: json extract shorthand
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  data->>'$.username' AS username,
  data->>'$.email' AS email
FROM json_test_data
WHERE json_test_data.name = 'user_profile'
`

// Test 3: Nested JSON path
const jsonNestedPath = sql`
-- @db: db_mysql
-- @name: json nested path
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  JSON_UNQUOTE(JSON_EXTRACT(data, '$.address.city')) AS city,
  JSON_UNQUOTE(JSON_EXTRACT(data, '$.address.zipCode')) AS zip_code
FROM json_test_data
WHERE json_test_data.name = 'user_with_address'
`

// Test 4: JSON array index access
const jsonArrayIndex = sql`
-- @db: db_mysql
-- @name: json array index
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  JSON_UNQUOTE(JSON_EXTRACT(data, '$.items[0].name')) AS first_item_name,
  CAST(JSON_EXTRACT(data, '$.items[0].price') AS DECIMAL(10,2)) AS first_item_price
FROM json_test_data
WHERE json_test_data.name = 'shopping_cart'
`

// Test 5: JSON_LENGTH for array length
const jsonArrayLength = sql`
-- @db: db_mysql
-- @name: json array length
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  JSON_LENGTH(JSON_EXTRACT(data, '$.tags')) AS tags_count
FROM json_test_data
WHERE json_test_data.name = 'tags'
`

// Test 6: JSON_TYPE
const jsonType = sql`
-- @db: db_mysql
-- @name: json type
SELECT
  json_test_data.id AS id,
  JSON_TYPE(JSON_EXTRACT(data, '$.username')) AS username_type,
  JSON_TYPE(JSON_EXTRACT(data, '$.age')) AS age_type,
  JSON_TYPE(JSON_EXTRACT(data, '$.tags')) AS tags_type
FROM json_test_data
WHERE json_test_data.name IN ('user_profile', 'tags')
`

// Test 7: JSON_CONTAINS for containment check
const jsonContains = sql`
-- @db: db_mysql
-- @name: json contains
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  JSON_CONTAINS(data, 'true', '$.active') AS is_active
FROM json_test_data
WHERE json_test_data.name = 'user_profile'
`

// Test 8: JSON_KEYS to get object keys
const jsonKeys = sql`
-- @db: db_mysql
-- @name: json keys
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  JSON_KEYS(data) AS all_keys
FROM json_test_data
WHERE json_test_data.name = 'user_profile'
`

// Test 9: JSON_OBJECT to build objects with type inference
const jsonObjectBuild = sql`
-- @db: db_mysql
-- @name: json object build
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  JSON_OBJECT(
    'id', json_test_data.id,
    'name', json_test_data.name,
    'username', JSON_EXTRACT(data, '$.username'),
    'email', JSON_EXTRACT(data, '$.email')
  ) AS user_summary
FROM json_test_data
WHERE json_test_data.name = 'user_profile'
`

// Test 10: Filter using JSON values
const jsonFilter = sql`
-- @db: db_mysql
-- @name: json filter
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  JSON_UNQUOTE(JSON_EXTRACT(data, '$.username')) AS username
FROM json_test_data
WHERE CAST(JSON_EXTRACT(data, '$.active') AS UNSIGNED) = 1
  AND CAST(JSON_EXTRACT(data, '$.age') AS UNSIGNED) > 25
`

// Test 11: Deep nested path
const jsonDeepPath = sql`
-- @db: db_mysql
-- @name: json deep path
SELECT
  json_test_data.id AS id,
  JSON_UNQUOTE(JSON_EXTRACT(data, '$.app.name')) AS app_name,
  JSON_UNQUOTE(JSON_EXTRACT(data, '$.app.settings.database.host')) AS db_host,
  CAST(JSON_EXTRACT(data, '$.app.settings.database.port') AS UNSIGNED) AS db_port
FROM json_test_data
WHERE json_test_data.name = 'nested_config'
`

// Test 12: JSON_VALID
const jsonValid = sql`
-- @db: db_mysql
-- @name: json valid
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  JSON_VALID(data) AS is_valid_json
FROM json_test_data
LIMIT 3
`

// Test 13: JSON_SEARCH to find values
const jsonSearch = sql`
-- @db: db_mysql
-- @name: json search
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  JSON_SEARCH(data, 'one', 'john_doe') AS username_path
FROM json_test_data
WHERE json_test_data.name = 'user_profile'
`
