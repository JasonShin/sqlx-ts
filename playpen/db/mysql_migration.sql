CREATE TABLE tables (
    id INTEGER NOT NULL AUTO_INCREMENT,
    number INTEGER NOT NULL,
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

INSERT INTO tables (number) VALUES
    (1), (2), (3), (4), (5), (6), (7), (8), (9), (10);

