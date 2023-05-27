CREATE TABLE tables (
    id INTEGER NOT NULL AUTO_INCREMENT,
    number INTEGER NOT NULL,
    occupied BOOL NOT NULL DEFAULT FALSE,
    PRIMARY KEY (id)
);

CREATE TABLE items (
   id INTEGER NOT NULL,
   food_type VARCHAR(30) NOT NULL,
   time_takes_to_cook INTEGER NOT NULL,
   table_id INTEGER NOT NULL,
   points SMALLINT NOT NULL,
   FOREIGN KEY (table_id) REFERENCES tables (id),
   PRIMARY KEY (id)
);

INSERT INTO tables (number)
VALUES
    (1), (2), (3), (4), (5), (6), (7), (8), (9), (10);

INSERT INTO items (id, food_type, time_takes_to_cook, table_id, points)
VALUES
  (1, 'korean', 10, 1, 2),
  (2, 'chinese', 10, 1, 2),
  (3, 'japanese', 10, 1, 2),
  (4, 'italian', 10, 1, 2),
  (5, 'french', 10, 1, 2);


-- We can primarily use this table to check how a column in MySQL can be converted to a TsFieldType

CREATE TABLE random (
	-- numeric types
    intz INT,
    smallint1 SMALLINT,
    tinyint1 TINYINT,
	medium1 MEDIUMINT,
	bigint1 BIGINT,
	decimal1 DECIMAL(2, 2),
	numeric1 NUMERIC(2, 2),
	double_precision1 DOUBLE PRECISION(2, 2),
	float1 FLOAT,
	double1 DOUBLE,
	bit1 BIT(2),
	bool1 BOOL,
	bool2 BOOLEAN,
	
	-- date and datetime types
	date1 DATE,
	datetime1 DATETIME,
	timestamp1 TIMESTAMP,
	year1 YEAR,
	
	-- string types
	char1 CHAR,
	varchar1 VARCHAR(20),
	binary1 BINARY,
	varbinary1 VARBINARY(2),
	blob1 BLOB,
	text1 TEXT,
		-- ideally this one should be generated as a legit enum type
	enum1 ENUM('x-small', 'small', 'medium', 'large', 'x-large'),
	set1 SET('one', 'two'),

	-- JSON types
	json1 JSON
	
);

