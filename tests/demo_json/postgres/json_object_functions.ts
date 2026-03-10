import { sql } from 'sqlx-ts'

// Test jsonb_object_keys - get all keys from object
const jsonbObjectKeys = sql`
-- @name: jsonb object keys
SELECT
  json_test_data.id,
  json_test_data.name,
  jsonb_object_keys(json_test_data.data) AS object_key
FROM json_test_data
WHERE json_test_data.name = 'user_profile'
`

// Test jsonb_typeof - get type of JSON value
const jsonbTypeof = sql`
-- @name: jsonb typeof
SELECT
  json_test_data.id,
  json_test_data.name,
  jsonb_typeof(json_test_data.data -> 'username') AS username_type,
  jsonb_typeof(json_test_data.data -> 'age') AS age_type,
  jsonb_typeof(json_test_data.data -> 'active') AS active_type,
  jsonb_typeof(json_test_data.data -> 'items') AS items_type,
  jsonb_typeof(json_test_data.data -> 'tags') AS tags_type
FROM json_test_data
WHERE json_test_data.name IN ('user_profile', 'shopping_cart', 'tags')
`

// Test jsonb_strip_nulls - remove null values
const jsonbStripNulls = sql`
-- @name: jsonb strip nulls
SELECT
  json_test_data.id,
  json_test_data.name,
  json_test_data.data -> 'reviews' -> 1 AS review_with_nulls,
  jsonb_strip_nulls(json_test_data.data -> 'reviews' -> 1) AS review_without_nulls
FROM json_test_data
WHERE json_test_data.name = 'product_reviews'
`

// Test ? operator - key exists
const jsonbKeyExists = sql`
-- @name: jsonb key exists
SELECT
  json_test_data.id,
  json_test_data.name,
  json_test_data.data ? 'username' AS has_username,
  json_test_data.data ? 'address' AS has_address,
  json_test_data.data ? 'nonexistent' AS has_nonexistent
FROM json_test_data
WHERE json_test_data.name IN ('user_profile', 'user_with_address')
`

// Test ?| operator - any key exists
const jsonbAnyKeyExists = sql`
-- @name: jsonb any key exists
SELECT
  json_test_data.id,
  json_test_data.name,
  json_test_data.data ?| array['username', 'email', 'phone'] AS has_any_contact
FROM json_test_data
WHERE json_test_data.name IN ('user_profile', 'user_with_address')
`

// Test ?& operator - all keys exist
const jsonbAllKeysExist = sql`
-- @name: jsonb all keys exist
SELECT
  json_test_data.id,
  json_test_data.name,
  json_test_data.data ?& array['username', 'email'] AS has_all_required,
  json_test_data.data ?& array['username', 'email', 'phone'] AS has_all_with_phone
FROM json_test_data
WHERE json_test_data.name IN ('user_profile', 'user_with_address')
`

// Test @> operator - contains (left contains right)
const jsonbContains = sql`
-- @name: jsonb contains
SELECT
  json_test_data.id,
  json_test_data.name,
  json_test_data.data @> '{"username": "john_doe"}'::jsonb AS has_specific_username,
  json_test_data.data @> '{"active": true}'::jsonb AS is_active
FROM json_test_data
WHERE json_test_data.name = 'user_profile'
`

// Test <@ operator - is contained by (left is contained in right)
const jsonbContainedBy = sql`
-- @name: jsonb contained by
SELECT
  json_test_data.id,
  json_test_data.name,
  '{"username": "john_doe"}'::jsonb <@ json_test_data.data AS username_in_data,
  '{"username": "john_doe", "age": 30}'::jsonb <@ json_test_data.data AS subset_in_data
FROM json_test_data
WHERE json_test_data.name = 'user_profile'
`

// Test jsonb_set - update value in JSON
const jsonbSet = sql`
-- @name: jsonb set
SELECT
  json_test_data.id,
  json_test_data.name,
  json_test_data.data AS original_data,
  jsonb_set(json_test_data.data, '{age}', '31') AS updated_age,
  jsonb_set(json_test_data.data, '{address, city}', '"New York"') AS updated_city
FROM json_test_data
WHERE json_test_data.name IN ('user_profile', 'user_with_address')
LIMIT 2
`

// Test jsonb_insert - insert value into JSON
const jsonbInsert = sql`
-- @name: jsonb insert
SELECT
  json_test_data.id,
  json_test_data.name,
  json_test_data.data AS original_data,
  jsonb_insert(json_test_data.data, '{phone}', '"555-1234"') AS with_phone
FROM json_test_data
WHERE json_test_data.name = 'user_profile'
`

