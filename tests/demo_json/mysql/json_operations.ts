import { sql } from 'sqlx-ts'

// JSON operators in SELECT - extract values
const jsonOperatorsSelect = sql`
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
