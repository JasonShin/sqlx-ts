import { sql } from 'sqlx-ts'

// jsonb_build_object basic
const jsonbBuildObjectBasic = sql`
-- @name: jsonb build object basic
SELECT
  id,
  jsonb_build_object('id', id, 'name', name, 'rarity', rarity) AS item_json
FROM items
`

// jsonb_agg for aggregation
const jsonbAggregation = sql`
-- @name: jsonb aggregation
SELECT
  rarity,
  jsonb_agg(jsonb_build_object('id', id, 'name', name)) AS items
FROM items
GROUP BY rarity
`

// JSON operators in SELECT
const jsonOperatorsSelect = sql`
-- @name: json operators select
SELECT
  id,
  name,
  jsonb_build_object('id', id, 'name', name) ->> 'name' AS extracted_name
FROM items
`
