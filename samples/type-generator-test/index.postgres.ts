import { sql } from "sqlx-ts";

const testQuery2 = sql`
-- @name: some query 2
-- @db: default
SELECT
    id,
    EXISTS(SELECT 1 FROM items WHERE points > 0) AS test_name
FROM items
WHERE id = $1
AND points > $2
AND points < $3;
`
