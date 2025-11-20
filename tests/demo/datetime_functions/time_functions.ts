import { sql } from 'sqlx-ts'

// NOW function
const nowFunction = sql`
-- @name: now function
SELECT
  id,
  name,
  NOW() AS current_time,
  CURRENT_TIME AS current_time_only
FROM characters
`

// Time comparison
const timeComparison = sql`
-- @name: time comparison
SELECT
  id,
  name,
  login_time,
  logout_time
FROM characters
WHERE login_time IS NOT NULL AND logout_time IS NOT NULL
  AND logout_time > login_time
`

// Interval operations
const intervalOperations = sql`
-- @name: interval operations
SELECT
  id,
  name,
  created_at,
  created_at + INTERVAL '1 hour' AS one_hour_later,
  created_at - INTERVAL '30 minutes' AS thirty_minutes_ago
FROM characters
WHERE created_at IS NOT NULL
`

// Date difference
const dateDifference = sql`
-- @name: date difference
SELECT
  id,
  name,
  logout_time - login_time AS session_duration
FROM characters
WHERE login_time IS NOT NULL AND logout_time IS NOT NULL
`
