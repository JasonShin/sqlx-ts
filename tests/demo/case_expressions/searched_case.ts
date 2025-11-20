import { sql } from 'sqlx-ts'

// Searched CASE basic
const searchedCaseBasic = sql`
-- @name: searched case basic
SELECT
  id,
  name,
  CASE
    WHEN id < 5 THEN 'low'
    WHEN id < 10 THEN 'medium'
    ELSE 'high'
  END AS id_category
FROM items
`

// Searched CASE with multiple conditions
const searchedCaseMultipleConditions = sql`
-- @name: searched case multiple conditions
SELECT
  id,
  name,
  rarity,
  CASE
    WHEN rarity = 'legendary' AND id > 10 THEN 'premium'
    WHEN rarity = 'legendary' THEN 'special'
    WHEN rarity = 'rare' THEN 'good'
    ELSE 'standard'
  END AS item_class
FROM items
`

// Searched CASE with parameters
const searchedCaseWithParams = sql`
-- @name: searched case with params
SELECT
  id,
  name,
  CASE
    WHEN id > $1 THEN 'above threshold'
    WHEN id = $1 THEN 'at threshold'
    ELSE 'below threshold'
  END AS threshold_status
FROM items
`
