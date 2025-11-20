import { sql } from 'sqlx-ts'

// LAG basic
const basicLag = sql`
-- @name: basic lag
SELECT
  id,
  name,
  LAG(name) OVER (ORDER BY id) AS previous_name
FROM items
`

// LEAD basic
const basicLead = sql`
-- @name: basic lead
SELECT
  id,
  name,
  LEAD(name) OVER (ORDER BY id) AS next_name
FROM items
`

// LAG with offset and default
const lagWithDefault = sql`
-- @name: lag with default
SELECT
  id,
  name,
  LAG(name, 1, 'N/A') OVER (ORDER BY id) AS previous_name
FROM items
`

// LAG and LEAD together
const lagAndLead = sql`
-- @name: lag and lead
SELECT
  id,
  name,
  LAG(name) OVER (ORDER BY id) AS previous_name,
  LEAD(name) OVER (ORDER BY id) AS next_name
FROM items
`

// LAG with PARTITION BY
const lagWithPartition = sql`
-- @name: lag with partition
SELECT
  id,
  name,
  rarity,
  LAG(name) OVER (PARTITION BY rarity ORDER BY id) AS previous_in_rarity
FROM items
`

// FIRST_VALUE and LAST_VALUE
const firstLastValue = sql`
-- @name: first last value
SELECT
  id,
  name,
  rarity,
  FIRST_VALUE(name) OVER (PARTITION BY rarity ORDER BY id) AS first_in_rarity,
  LAST_VALUE(name) OVER (PARTITION BY rarity ORDER BY id ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING) AS last_in_rarity
FROM items
`
