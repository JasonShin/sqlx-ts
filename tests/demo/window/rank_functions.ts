import { sql } from 'sqlx-ts'

// RANK basic
const basicRank = sql`
-- @name: basic rank
SELECT
  id,
  name,
  rarity,
  RANK() OVER (ORDER BY id) AS rank
FROM items
`

// DENSE_RANK
const densRank = sql`
-- @name: dense rank
SELECT
  id,
  name,
  rarity,
  DENSE_RANK() OVER (ORDER BY id DESC) AS dense_rank
FROM items
`

// RANK with PARTITION BY
const rankWithPartition = sql`
-- @name: rank with partition
SELECT
  id,
  name,
  rarity,
  RANK() OVER (PARTITION BY rarity ORDER BY id) AS rank
FROM items
`

// Multiple ranking functions
const multipleRanking = sql`
-- @name: multiple ranking
SELECT
  id,
  name,
  rarity,
  ROW_NUMBER() OVER (ORDER BY id) AS row_num,
  RANK() OVER (ORDER BY id) AS rank,
  DENSE_RANK() OVER (ORDER BY id) AS dense_rank
FROM items
`

// NTILE for quartiles
const ntileQuartiles = sql`
-- @name: ntile quartiles
SELECT
  id,
  name,
  NTILE(4) OVER (ORDER BY id) AS quartile
FROM items
`
