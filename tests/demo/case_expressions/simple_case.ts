import { sql } from 'sqlx-ts'

// Simple CASE expression
const simpleCaseBasic = sql`
-- @name: simple case basic
SELECT
  id,
  name,
  CASE rarity
    WHEN 'common' THEN 'C'
    WHEN 'rare' THEN 'R'
    WHEN 'legendary' THEN 'L'
    ELSE 'U'
  END AS rarity_code
FROM items
`

// Simple CASE with NULL handling
const simpleCaseWithNull = sql`
-- @name: simple case with null
SELECT
  id,
  name,
  CASE rarity
    WHEN 'common' THEN 1
    WHEN 'rare' THEN 2
    WHEN 'legendary' THEN 3
    ELSE 0
  END AS rarity_level
FROM items
`

// Multiple CASE expressions
const multipleCaseExpressions = sql`
-- @name: multiple case expressions
SELECT
  id,
  name,
  CASE rarity
    WHEN 'common' THEN 'low'
    WHEN 'rare' THEN 'medium'
    ELSE 'high'
  END AS rarity_tier,
  CASE
    WHEN id < 5 THEN 'early'
    WHEN id < 10 THEN 'mid'
    ELSE 'late'
  END AS id_range
FROM items
`
