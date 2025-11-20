import { sql } from 'sqlx-ts'

// Basic arithmetic operators
const arithmeticOperators = sql`
-- @name: arithmetic operators
SELECT
  id,
  name,
  id + 10 AS id_plus_10,
  id - 5 AS id_minus_5,
  id * 2 AS id_times_2,
  id / 2 AS id_divided_2,
  id % 3 AS id_mod_3
FROM items
`

// Mathematical functions
const mathFunctions = sql`
-- @name: math functions
SELECT
  id,
  name,
  ABS(id - 10) AS distance_from_10,
  POWER(id, 2) AS id_squared,
  SQRT(id) AS id_sqrt,
  CEIL(id::numeric / 3) AS id_div_3_ceil,
  FLOOR(id::numeric / 3) AS id_div_3_floor,
  ROUND(id::numeric / 3, 2) AS id_div_3_rounded
FROM items
`

// Comparison operators
const comparisonOperators = sql`
-- @name: comparison operators
SELECT
  id,
  name,
  id > 5 AS is_above_5,
  id >= 5 AS is_at_least_5,
  id < 5 AS is_below_5,
  id <= 5 AS is_at_most_5,
  id = 5 AS is_exactly_5,
  id != 5 AS is_not_5
FROM items
`

// BETWEEN and IN operators
const betweenAndIn = sql`
-- @name: between and in
SELECT
  id,
  name,
  rarity,
  id BETWEEN 3 AND 7 AS is_in_range,
  rarity IN ('common', 'rare') AS is_common_or_rare
FROM items
`
