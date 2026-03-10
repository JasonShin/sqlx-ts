import { sql } from 'sqlx-ts'


// Test JSON_LENGTH - get array length
const jsonArrayLength = sql`
-- @db: db_mysql
-- @name: json array length
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  JSON_LENGTH(data, '$.items') AS items_count,
  JSON_LENGTH(data, '$.tags') AS tags_count
FROM json_test_data
WHERE json_test_data.name IN ('shopping_cart', 'tags')
`

// Test JSON_EXTRACT with array index
const jsonArrayExtract = sql`
-- @db: db_mysql
-- @name: json array extract
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  JSON_EXTRACT(data, '$.tags[0]') AS first_tag,
  JSON_EXTRACT(data, '$.tags[1]') AS second_tag,
  JSON_EXTRACT(data, '$.tags[2]') AS third_tag
FROM json_test_data
WHERE json_test_data.name = 'tags'
`

// Test array contains using JSON_CONTAINS
const jsonArrayContains = sql`
-- @db: db_mysql
-- @name: json array contains
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  JSON_EXTRACT(data, '$.tags') AS tags,
  JSON_CONTAINS(JSON_EXTRACT(data, '$.tags'), JSON_QUOTE('mysql')) AS has_mysql,
  JSON_CONTAINS(JSON_EXTRACT(data, '$.tags'), JSON_QUOTE('database')) AS has_database
FROM json_test_data
WHERE json_test_data.name = 'tags'
`

// Test array element membership
const jsonArrayMembership = sql`
-- @db: db_mysql
-- @name: json array membership
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  JSON_CONTAINS(JSON_EXTRACT(data, '$.tags'), JSON_QUOTE('mysql')) AS has_mysql_tag,
  JSON_CONTAINS(JSON_EXTRACT(data, '$.tags'), JSON_QUOTE('tutorial')) AS has_tutorial_tag
FROM json_test_data
WHERE json_test_data.name = 'tags'
`

// Test nested array access
const jsonNestedArrayAccess = sql`
-- @db: db_mysql
-- @name: json nested array access
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  JSON_EXTRACT(data, '$.items[0].name') AS first_item_name,
  JSON_EXTRACT(data, '$.items[0].price') AS first_item_price,
  JSON_EXTRACT(data, '$.items[1].name') AS second_item_name,
  JSON_EXTRACT(data, '$.items[1].quantity') AS second_item_quantity
FROM json_test_data
WHERE json_test_data.name = 'shopping_cart'
`

// Test deep nested array
const jsonDeepNestedArray = sql`
-- @db: db_mysql
-- @name: json deep nested array
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  JSON_EXTRACT(data, '$.stats.inventory[0].item') AS first_inventory_item,
  JSON_EXTRACT(data, '$.stats.inventory[0].rarity') AS first_item_rarity,
  JSON_EXTRACT(data, '$.stats.inventory[1].item') AS second_inventory_item,
  JSON_EXTRACT(data, '$.stats.achievements[0]') AS first_achievement
FROM json_test_data
WHERE json_test_data.name = 'game_stats'
`

// Test JSON_ARRAY to build arrays
const jsonArrayBuild = sql`
-- @db: db_mysql
-- @name: json array build
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  JSON_ARRAY(
    JSON_EXTRACT(data, '$.tags[0]'),
    JSON_EXTRACT(data, '$.tags[1]')
  ) AS first_two_tags
FROM json_test_data
WHERE json_test_data.name = 'tags'
`

// Test JSON_ARRAY_APPEND to add elements
const jsonArrayAppend = sql`
-- @db: db_mysql
-- @name: json array append
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  JSON_EXTRACT(data, '$.tags') AS original_tags,
  JSON_ARRAY_APPEND(JSON_EXTRACT(data, '$.tags'), '$', 'new_tag') AS tags_with_new
FROM json_test_data
WHERE json_test_data.name = 'tags'
`

// Test JSON_ARRAY_INSERT to insert elements
const jsonArrayInsert = sql`
-- @db: db_mysql
-- @name: json array insert
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  JSON_EXTRACT(data, '$.tags') AS original_tags,
  JSON_ARRAY_INSERT(JSON_EXTRACT(data, '$.tags'), '$[1]', 'inserted_tag') AS tags_with_insert
FROM json_test_data
WHERE json_test_data.name = 'tags'
`
