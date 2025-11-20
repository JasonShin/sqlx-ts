import { sql } from 'sqlx-ts'

// CURRENT_DATE and CURRENT_TIMESTAMP
const currentDateTimestamp = sql`
-- @name: current date timestamp
SELECT
  id,
  name,
  CURRENT_DATE AS today,
  CURRENT_TIMESTAMP AS now
FROM characters
`

// DATE_TRUNC function
const dateTrunc = sql`
-- @name: date trunc
SELECT
  id,
  name,
  DATE_TRUNC('day', created_at) AS created_day,
  DATE_TRUNC('month', created_at) AS created_month,
  DATE_TRUNC('year', created_at) AS created_year
FROM characters
WHERE created_at IS NOT NULL
`

// EXTRACT function
const extractFunction = sql`
-- @name: extract function
SELECT
  id,
  name,
  EXTRACT(YEAR FROM created_at) AS created_year,
  EXTRACT(MONTH FROM created_at) AS created_month,
  EXTRACT(DAY FROM created_at) AS created_day
FROM characters
WHERE created_at IS NOT NULL
`

// AGE function
const ageFunction = sql`
-- @name: age function
SELECT
  id,
  name,
  AGE(created_at) AS account_age,
  AGE(CURRENT_TIMESTAMP, created_at) AS account_age_explicit
FROM characters
WHERE created_at IS NOT NULL
`

// Date arithmetic
const dateArithmetic = sql`
-- @name: date arithmetic
SELECT
  id,
  name,
  created_at,
  created_at + INTERVAL '7 days' AS one_week_later,
  created_at - INTERVAL '1 month' AS one_month_ago
FROM characters
WHERE created_at IS NOT NULL
`
