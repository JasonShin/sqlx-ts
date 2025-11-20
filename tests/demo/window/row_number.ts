import { sql } from 'sqlx-ts'

// Basic ROW_NUMBER
const basicRowNumber = sql`
-- @name: basic row number
SELECT
  id,
  name,
  rarity,
  ROW_NUMBER() OVER (ORDER BY id) AS row_num
FROM items
`

// ROW_NUMBER with PARTITION BY
const rowNumberWithPartition = sql`
-- @name: row number with partition
SELECT
  id,
  name,
  rarity,
  ROW_NUMBER() OVER (PARTITION BY rarity ORDER BY id) AS row_num
FROM items
`

// ROW_NUMBER with WHERE
const rowNumberWithWhere = sql`
-- @name: row number with where
SELECT
  id,
  name,
  rarity,
  ROW_NUMBER() OVER (ORDER BY name) AS row_num
FROM items
WHERE rarity IS NOT NULL
`

// ROW_NUMBER with parameters
const rowNumberWithParams = sql`
-- @name: row number with params
SELECT
  id,
  name,
  rarity,
  ROW_NUMBER() OVER (ORDER BY id) AS row_num
FROM items
WHERE rarity = $1
`
