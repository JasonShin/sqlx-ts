import { sql } from 'sqlx-ts'


// Test JSON_KEYS - get all keys from object
const jsonObjectKeys = sql`
-- @db: db_mysql
-- @name: json object keys
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  JSON_KEYS(data) AS object_keys
FROM json_test_data
WHERE json_test_data.name = 'user_profile'
`

// Test JSON_KEYS with path
const jsonObjectKeysPath = sql`
-- @db: db_mysql
-- @name: json object keys path
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  JSON_KEYS(data, '$.address') AS address_keys
FROM json_test_data
WHERE json_test_data.name = 'user_with_address'
`

// Test JSON_TYPE - get type of JSON value
const jsonTypeof = sql`
-- @db: db_mysql
-- @name: json typeof
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  JSON_TYPE(JSON_EXTRACT(data, '$.username')) AS username_type,
  JSON_TYPE(JSON_EXTRACT(data, '$.age')) AS age_type,
  JSON_TYPE(JSON_EXTRACT(data, '$.active')) AS active_type,
  JSON_TYPE(JSON_EXTRACT(data, '$.items')) AS items_type,
  JSON_TYPE(JSON_EXTRACT(data, '$.tags')) AS tags_type
FROM json_test_data
WHERE json_test_data.name IN ('user_profile', 'shopping_cart', 'tags')
`

// Test JSON_CONTAINS - check if JSON contains value
const jsonContains = sql`
-- @db: db_mysql
-- @name: json contains
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  JSON_CONTAINS(data, JSON_QUOTE('john_doe'), '$.username') AS has_specific_username,
  JSON_CONTAINS(data, 'true', '$.active') AS is_active
FROM json_test_data
WHERE json_test_data.name = 'user_profile'
`

// Test JSON_CONTAINS_PATH - check if path exists
const jsonContainsPath = sql`
-- @db: db_mysql
-- @name: json contains path
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  JSON_CONTAINS_PATH(data, 'one', '$.username') AS has_username,
  JSON_CONTAINS_PATH(data, 'one', '$.address') AS has_address,
  JSON_CONTAINS_PATH(data, 'one', '$.nonexistent') AS has_nonexistent,
  JSON_CONTAINS_PATH(data, 'all', '$.username', '$.email') AS has_both
FROM json_test_data
WHERE json_test_data.name IN ('user_profile', 'user_with_address')
`

// Test JSON_OBJECT - build JSON objects
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

// Test JSON_SET - update value in JSON
const jsonSet = sql`
-- @db: db_mysql
-- @name: json set
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  data AS original_data,
  JSON_SET(data, '$.age', 31) AS updated_age,
  JSON_SET(data, '$.address.city', 'New York') AS updated_city
FROM json_test_data
WHERE json_test_data.name IN ('user_profile', 'user_with_address')
LIMIT 2
`

// Test JSON_INSERT - insert value into JSON
const jsonInsert = sql`
-- @db: db_mysql
-- @name: json insert
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  data AS original_data,
  JSON_INSERT(data, '$.phone', '555-1234') AS with_phone
FROM json_test_data
WHERE json_test_data.name = 'user_profile'
`

// Test JSON_REPLACE - replace existing value
const jsonReplace = sql`
-- @db: db_mysql
-- @name: json replace
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  data AS original_data,
  JSON_REPLACE(data, '$.username', 'new_username') AS with_new_username
FROM json_test_data
WHERE json_test_data.name = 'user_profile'
`

// Test JSON_REMOVE - remove keys from JSON
const jsonRemove = sql`
-- @db: db_mysql
-- @name: json remove
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  data AS original_data,
  JSON_REMOVE(data, '$.age') AS without_age
FROM json_test_data
WHERE json_test_data.name = 'user_profile'
`

// Test JSON_MERGE_PATCH - merge JSON objects
const jsonMergePatch = sql`
-- @db: db_mysql
-- @name: json merge patch
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  data AS original_data,
  JSON_MERGE_PATCH(data, JSON_OBJECT('verified', true, 'lastLogin', '2024-01-15')) AS merged_data
FROM json_test_data
WHERE json_test_data.name = 'user_profile'
`

// Test JSON_MERGE_PRESERVE - merge preserving all values
const jsonMergePreserve = sql`
-- @db: db_mysql
-- @name: json merge preserve
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  data AS original_data,
  JSON_MERGE_PRESERVE(data, JSON_OBJECT('newField', 'newValue')) AS merged_data
FROM json_test_data
WHERE json_test_data.name = 'user_profile'
`

// Test JSON_SEARCH - find values in JSON
const jsonSearch = sql`
-- @db: db_mysql
-- @name: json search
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  JSON_SEARCH(data, 'one', 'john_doe') AS username_path,
  JSON_SEARCH(data, 'one', 'john@example.com') AS email_path
FROM json_test_data
WHERE json_test_data.name = 'user_profile'
`

// Test JSON_DEPTH - get depth of JSON
const jsonDepth = sql`
-- @db: db_mysql
-- @name: json depth
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  JSON_DEPTH(data) AS data_depth
FROM json_test_data
WHERE json_test_data.name IN ('user_profile', 'nested_config')
`

// Test JSON_VALID - validate JSON
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
