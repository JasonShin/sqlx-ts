import { sql } from 'sqlx-ts'

// Test jsonb_array_length - get array length
const jsonbArrayLength = sql`
-- @name: jsonb array length
SELECT
  json_test_data.id,
  json_test_data.name,
  jsonb_array_length(data -> 'items') AS items_count,
  jsonb_array_length(data -> 'tags') AS tags_count
FROM json_test_data
WHERE json_test_data.name IN ('shopping_cart', 'tags')
`

// Test array contains using @> operator
const jsonbArrayContains = sql`
-- @name: jsonb array contains
SELECT
  json_test_data.id,
  json_test_data.name,
  data -> 'tags' AS tags
FROM json_test_data
WHERE data -> 'tags' @> '["postgresql"]'::jsonb
`

