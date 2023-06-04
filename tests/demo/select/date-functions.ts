import { sql } from 'sqlx-ts'

const atTimeZone = sql`
SELECT
    id,
    '2018-09-02 07:09:19'::timestamp AT TIME ZONE 'America/Chicago' as some_date
FROM items;
`

const cast = sql`
SELECT
	id,
	CAST('2015-01-01' AS DATE) as DATE
FROM items;
`

const extract = sql`
SELECT
    id,
    EXTRACT(MONTH FROM DATE '2017-08-08') AS THE_MONTH
FROM items;
`
