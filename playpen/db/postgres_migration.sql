DROP SCHEMA public CASCADE;
CREATE SCHEMA public;

GRANT ALL ON SCHEMA public TO postgres;
GRANT ALL ON SCHEMA public TO public;

CREATE TABLE postgres.public.tables (
    id SERIAL NOT NULL,
    number INTEGER NOT NULL,
    occupied BOOL NOT NULL DEFAULT FALSE,
    PRIMARY KEY (id)
);

CREATE TABLE postgres.public.items (
    id SERIAL NOT NULL,
    food_type VARCHAR(30) NOT NULL,
    time_takes_to_cook INTEGER NOT NULL,
    table_id INTEGER NOT NULL,
    points SMALLINT NOT NULL,
    FOREIGN KEY (table_id) REFERENCES public.tables (id),
    PRIMARY KEY (id)
);

-- A table of randomness, just to test various field types in PostgreSQL
-- There is a pretty comprehensive list of data types available in Postgres
-- found in https://www.geeksforgeeks.org/postgresql-data-types/ -> not the official Postgres doc
CREATE TABLE postgres.public.random (
	-- Strings
	char1 CHAR(2),
	varchar1 VARCHAR(20),
	tinyblob1 bytea,
	text1 TEXT,

    -- Numeric
    smallint1 SMALLINT NULL,
    int1 INTEGER NULL,
    serial1 SERIAL,
    -- Floating-point number
    float1 FLOAT(2) NULL,
    float2 FLOAT8 NULL,
    float3 REAL NULL,
    float5 NUMERIC(2, 1) NULL,
    -- Temporal data type
    date1 DATE null,
    time1 TIME null,
    time2 TIMESTAMP null,
    time3 TIMESTAMPTZ null, 
   	time4 interval null,
   	
   	-- array
   	array1 integer[3][3],
   	array2 boolean[2],
   	array3 TIME[2],
   	
   	-- json
   	json1 JSON,
   	json2 JSONB,
   	
   	-- UUID
   	uuid1 UUID,
   	
    -- Special data types
   	box1 BOX,
   	point1 POINT,
   	lseg1 LSEG,
   	polygon1 POLYGON,
   	inet1 INET,
   	macaddr1 MACADDR
);

INSERT INTO public.tables (number) VALUES
(1), (2), (3), (4), (5), (6), (7), (8), (9), (10);

INSERT INTO public.items (food_type, time_takes_to_cook, table_id, points)
VALUES
  ('korean', 10, 1, 2),
  ('chinese', 10, 1, 2),
  ('japanese', 10, 1, 2),
  ('italian', 10, 1, 2),
  ('french', 10, 1, 2);
