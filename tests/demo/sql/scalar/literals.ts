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
const dat1 = sql`SELECT NOW() - interval '3' day AS dat FROM items`
