import { sql } from 'sqlx-ts'

// Nested CASE basic
const nestedCaseBasic = sql`
-- @name: nested case basic
SELECT
  id,
  name,
  rarity,
  CASE rarity
    WHEN 'legendary' THEN
      CASE
        WHEN id > 10 THEN 'ultra rare'
        ELSE 'rare'
      END
    WHEN 'rare' THEN 'uncommon'
    ELSE 'common'
  END AS detailed_rarity
FROM items
`

// Nested CASE with multiple levels
const nestedCaseMultipleLevels = sql`
-- @name: nested case multiple levels
SELECT
  id,
  name,
  CASE
    WHEN rarity = 'legendary' THEN
      CASE
        WHEN id > 15 THEN 'S-tier'
        WHEN id > 10 THEN 'A-tier'
        ELSE 'B-tier'
      END
    WHEN rarity = 'rare' THEN
      CASE
        WHEN id > 10 THEN 'C-tier'
        ELSE 'D-tier'
      END
    ELSE 'E-tier'
  END AS tier
FROM items
`
