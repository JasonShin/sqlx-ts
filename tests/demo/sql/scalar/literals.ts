import { sql } from 'sqlx-ts'

const num1 = sql`SELECT 3 AS num FROM items`
const num2 = sql`SELECT 3. AS num FROM items`
const num3 = sql`SELECT 3.3 AS num FROM items`
const num4 = sql`SELECT .3 AS num FROM items`
const num5 = sql`SELECT 3.e3 AS num FROM items`
const num6 = sql`SELECT 3.3e3 AS num FROM items`
const num7 = sql`SELECT .3e3 AS num FROM items`
const num8 = sql`SELECT 3e3 AS num FROM items`
const num10 = sql`SELECT 3e+3 AS num FROM items`
const num11 = sql`SELECT 3e-3 AS num FROM items`
const str1 = sql`SELECT 'str' AS str FROM items`
const str2 = sql`SELECT '1' AS str FROM items`
const str3 = sql`SELECT '1' AS str FROM items`
// BINARY OP is being calculated weird
const dat1 = sql`SELECT NOW() - interval '3' day AS dat FROM items`
const dat2 = sql`SELECT NOW() - interval '3 weeks' AS dat FROM items`
// DOT
const dot1 = sql`SELECT items.id FROM items`
const dot2 = sql`SELECT items.* FROM items`
const dot3 = sql`SELECT items.id, items.* FROM items`
const dot4 = sql`SELECT ROW(items.*, 42) AS something FROM items`
// CASE EXPR
const case1 = sql`SELECT CASE WHEN true THEN 1 ELSE 0 END AS num FROM items`
// CONVERT - not supported yet
// const convert1 = sql`SELECT CONVERT(1, int) AS num FROM items`
// OPERATIONS
const operation1 = sql`SELECT id + id FROM items`
const operation2 = sql`SELECT id + id * id FROM items`
const operation3 = sql`SELECT id + id * id / id FROM items`
const operation4 = sql`SELECT id - id * id / id + id FROM items`
