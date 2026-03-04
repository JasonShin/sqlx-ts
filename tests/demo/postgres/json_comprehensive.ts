import { sql } from 'sqlx-ts'

// Test 1: JSON access operators -> and ->>
const jsonAccessOperators = sql`
-- @name: json access operators
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  data ->> 'username' AS username,
  data ->> 'email' AS email,
  (data ->> 'age')::integer AS age
FROM json_test_data
WHERE json_test_data.name = 'user_profile'
`

// Test 2: Nested JSON access with #> and #>>
const jsonNestedAccess = sql`
-- @name: json nested access
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  data #>> '{address, city}' AS city,
  data #>> '{address, zipCode}' AS zip_code
FROM json_test_data
WHERE json_test_data.name = 'user_with_address'
`

// Test 3: JSON array access by index
const jsonArrayIndex = sql`
-- @name: json array index
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  data -> 'items' -> 0 ->> 'name' AS first_item_name,
  (data -> 'items' -> 0 ->> 'price')::numeric AS first_item_price
FROM json_test_data
WHERE json_test_data.name = 'shopping_cart'
`

// Test 4: JSON array length
const jsonArrayLength = sql`
-- @name: json array length
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  jsonb_array_length(data -> 'tags') AS tags_count
FROM json_test_data
WHERE json_test_data.name = 'tags'
`

// Test 5: JSON typeof
const jsonTypeof = sql`
-- @name: json typeof
SELECT
  json_test_data.id AS id,
  jsonb_typeof(data -> 'username') AS username_type,
  jsonb_typeof(data -> 'age') AS age_type,
  jsonb_typeof(data -> 'tags') AS tags_type
FROM json_test_data
WHERE json_test_data.name IN ('user_profile', 'tags')
`

// Test 6: JSON key existence with ?
const jsonKeyExists = sql`
-- @name: json key exists
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  (data ? 'username')::text AS has_username,
  (data ? 'address')::text AS has_address
FROM json_test_data
WHERE json_test_data.name IN ('user_profile', 'user_with_address')
`

// Test 7: JSON containment with @>
const jsonContains = sql`
-- @name: json contains
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  (data @> '{"active": true}'::jsonb)::text AS is_active
FROM json_test_data
WHERE json_test_data.name = 'user_profile'
`

// Test 8: JSON build object with typed fields
const jsonBuildObjectTyped = sql`
-- @name: json build object typed
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  jsonb_build_object(
    'id', json_test_data.id,
    'name', json_test_data.name,
    'username', data ->> 'username',
    'email', data ->> 'email'
  ) AS user_summary
FROM json_test_data
WHERE json_test_data.name = 'user_profile'
`

// Test 9: Filter using JSON operators
const jsonFilter = sql`
-- @name: json filter
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  data ->> 'username' AS username
FROM json_test_data
WHERE (data ->> 'active')::boolean = true
  AND (data ->> 'age')::integer > 25
`

// Test 10: JSON path queries
const jsonDeepPath = sql`
-- @name: json deep path
SELECT
  json_test_data.id AS id,
  data #>> '{app, name}' AS app_name,
  data #>> '{app, settings, database, host}' AS db_host,
  (data #>> '{app, settings, database, port}')::integer AS db_port
FROM json_test_data
WHERE json_test_data.name = 'nested_config'
`
