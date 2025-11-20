import { sql } from 'sqlx-ts'

// CONCAT function
const concatFunction = sql`
-- @name: concat function
SELECT
  id,
  CONCAT(name, ' (', rarity, ')') AS display_name
FROM items
`

// UPPER and LOWER functions
const upperLowerFunctions = sql`
-- @name: upper lower functions
SELECT
  id,
  UPPER(name) AS name_upper,
  LOWER(name) AS name_lower
FROM items
`

// LENGTH function
const lengthFunction = sql`
-- @name: length function
SELECT
  id,
  name,
  LENGTH(name) AS name_length
FROM items
`

// SUBSTRING function
const substringFunction = sql`
-- @name: substring function
SELECT
  id,
  name,
  SUBSTRING(name FROM 1 FOR 3) AS name_prefix
FROM items
`

// TRIM functions
const trimFunctions = sql`
-- @name: trim functions
SELECT
  id,
  TRIM(name) AS name_trimmed,
  LTRIM(name) AS name_ltrim,
  RTRIM(name) AS name_rtrim
FROM items
`

// REPLACE function
const replaceFunction = sql`
-- @name: replace function
SELECT
  id,
  name,
  REPLACE(name, 'a', 'A') AS name_replaced
FROM items
`
