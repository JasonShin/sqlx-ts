import { sql } from 'sqlx-ts'

const ceil = sql`
SELECT
    id,
    CEIL(51.11) AS some_number
FROM items;
`

const all = sql`
select
	abs(1) as abs1,
	acos(1) as acos1,
	asin(1) as asin1,
	atan(1) as atan1,
	avg(1) as avg1,
	ceil(1) as ceil1,
	ceiling(1) as ceiling1,
	cos(1) as cos1,
	cot(1) as cot1,
	count(1) as count1,
	degrees(1) as degrees1,
	exp(1) as exp1,
	floor(1) as floor1,
	greatest(1) as greatest1,
	least(1) as least1,
	ln(1) as ln1,
	log(1) as log1,
	log10(1) as log101,
	max(1) as max1,
	min(1) as min1,
	pow(1, 1) as pow1,
	mod(1, 1) as mod1,
	pi() as pi1,
	power(1, 1) as pow2,
	radians(1) as radians1,
	round(1, 1) as round1,
	sign(1) as sign1,
	sin(1) as sin1,
	sqrt(1) as sqrt1,
	sum(1) as sum1,
	tan(1) as tan1,
	trunc(1.1) as trunc1
from items;
`
