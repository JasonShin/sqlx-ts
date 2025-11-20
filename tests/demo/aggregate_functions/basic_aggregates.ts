import { sql } from 'sqlx-ts'

// COUNT variants
const countVariants = sql`
-- @name: count variants
SELECT
  COUNT(*) AS total_count,
  COUNT(id) AS id_count,
  COUNT(DISTINCT rarity) AS distinct_rarity_count,
  COUNT(rarity) AS non_null_rarity_count
FROM items
`

// SUM and AVG
const sumAndAvg = sql`
-- @name: sum and avg
SELECT
  rarity,
  SUM(id) AS sum_id,
  AVG(id) AS avg_id,
  AVG(id)::INTEGER AS avg_id_rounded
FROM items
GROUP BY rarity
`

// MIN and MAX
const minAndMax = sql`
-- @name: min and max
SELECT
  rarity,
  MIN(id) AS min_id,
  MAX(id) AS max_id,
  MIN(name) AS min_name,
  MAX(name) AS max_name
FROM items
GROUP BY rarity
`

// STDDEV and VARIANCE
const stddevAndVariance = sql`
-- @name: stddev and variance
SELECT
  rarity,
  STDDEV(id) AS stddev_id,
  VARIANCE(id) AS variance_id
FROM items
GROUP BY rarity
HAVING COUNT(*) > 1
`

// Multiple aggregates combined
const multipleAggregates = sql`
-- @name: multiple aggregates
SELECT
  rarity,
  COUNT(*) AS count,
  SUM(id) AS sum_id,
  AVG(id) AS avg_id,
  MIN(id) AS min_id,
  MAX(id) AS max_id
FROM items
GROUP BY rarity
`
