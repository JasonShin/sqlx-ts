import { sql } from 'sqlx-ts'

// Single column partition
const singlePartition = sql`
-- @name: single partition
SELECT
  id,
  name,
  rarity,
  COUNT(*) OVER (PARTITION BY rarity) AS rarity_count
FROM items
`

// Multiple column partition
const multiplePartition = sql`
-- @name: multiple partition
SELECT
  id,
  name,
  rarity,
  COUNT(*) OVER (PARTITION BY rarity, name) AS count_per_group
FROM items
`

// Window aggregate functions
const windowAggregates = sql`
-- @name: window aggregates
SELECT
  id,
  name,
  rarity,
  COUNT(*) OVER (PARTITION BY rarity) AS count,
  SUM(id) OVER (PARTITION BY rarity) AS sum_id,
  AVG(id) OVER (PARTITION BY rarity) AS avg_id,
  MIN(id) OVER (PARTITION BY rarity) AS min_id,
  MAX(id) OVER (PARTITION BY rarity) AS max_id
FROM items
`

// Window with ORDER BY
const windowWithOrderBy = sql`
-- @name: window with order by
SELECT
  id,
  name,
  rarity,
  SUM(id) OVER (PARTITION BY rarity ORDER BY id) AS running_sum
FROM items
`
