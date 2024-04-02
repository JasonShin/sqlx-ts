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
// UNARY OPERATIONS
const unary1 = sql`SELECT -id FROM items`
const unary2 = sql`SELECT +id FROM items`
const unary4 = sql`SELECT NOT occupied FROM tables`
const unary5 = sql`SELECT NOT NOT occupied FROM tables`
// CAST
const cast1 = sql`SELECT CAST('1' AS int) AS num FROM items`
const cast2 = sql`SELECT CAST('1' AS double precision) AS num FROM items`
const cast3 = sql`SELECT CAST('1' AS float(8)) AS num FROM items`
const cast4 = sql`SELECT CAST('1' AS decimal(15,2)) AS num FROM items`
// UNARY OPERATORS
const unaryOp1 = sql`SELECT id IN (1, 2, 3) AS test FROM items`
const unaryOp2 = sql`SELECT id NOT IN (1, 2, 3) AS test FROM items`
const unaryOp3 = sql`SELECT id IS NULL AS test FROM items`
const unaryOp4 = sql`SELECT id IS NOT NULL AS test FROM items`
const unaryOp5 = sql`SELECT occupied IS TRUE AS test FROM tables`
const unaryOp6 = sql`SELECT occupied IS NOT TRUE AS test FROM tables`
const unaryOp7 = sql`SELECT occupied IS FALSE AS test FROM tables`
const unaryOp8 = sql`SELECT occupied IS NOT FALSE AS test FROM tables`
const unaryOp9 = sql`SELECT occupied IS UNKNOWN AS test FROM tables`
const unaryOp10 = sql`SELECT occupied IS NOT UNKNOWN AS test FROM tables`
const unaryOp11 = sql`SELECT NOT (id IN (1, 2, 3)) AS test FROM items`
const unaryOp12 = sql`SELECT id LIKE time_takes_to_cook AS test FROM items`
const unaryOp13 = sql`SELECT EXTRACT(DAY FROM created_at) AS some_day FROM items`
const unaryOp14 = sql`SELECT SUBSTRING(food_type, 5, 3) AS some_str FROM items`
const unaryOp15 = sql`SELECT POSITION('a' IN food_type) AS some_pos FROM items`
const unaryOp16 = sql`SELECT CHAR_LENGTH(food_type) AS some_len FROM items`
const unaryOp17 = sql`SELECT COALESCE(food_type, 'no type') AS some_type FROM items`
const unaryOp18 = sql`SELECT NULLIF(food_type, 'no type') AS some_type FROM items`
const unaryOp19 = sql`SELECT id IS DISTINCT FROM 1 AS test FROM items`
// BINARY OPERATORS
const binaryOp1 = sql`SELECT id = 1 AS test FROM items`
const binaryOp2 = sql`SELECT id <> 1 AS test FROM items`
const binaryOp3 = sql`SELECT id < 1 AS test FROM items`
const binaryOp4 = sql`SELECT id <= 1 AS test FROM items`
const binaryOp5 = sql`SELECT id > 1 AS test FROM items`
const binaryOp6 = sql`SELECT id >= 1 AS test FROM items`
const binaryOp7 = sql`SELECT id + 1 AS test FROM items`
const binaryOp8 = sql`SELECT id - 1 AS test FROM items`
const binaryOp9 = sql`SELECT id * 1 AS test FROM items`
const binaryOp10 = sql`SELECT id / 1 AS test FROM items`
const binaryOp11 = sql`SELECT id % 1 AS test FROM items`
const binaryOp13 = sql`SELECT id & 1 AS test FROM items`
const binaryOp14 = sql`SELECT id | 1 AS test FROM items`
const binaryOp16 = sql`SELECT id << 1 AS test FROM items`
const binaryOp17 = sql`SELECT id >> 1 AS test FROM items`
const binaryOp18 = sql`SELECT id ^ 1 AS test FROM items`
