CREATE TABLE IF NOT EXISTS tables (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    number INTEGER NOT NULL,
    occupied BOOLEAN NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS items (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    food_type TEXT NOT NULL,
    time_takes_to_cook INTEGER NOT NULL,
    table_id INTEGER NOT NULL,
    points INTEGER NOT NULL,
    FOREIGN KEY (table_id) REFERENCES tables (id)
);

CREATE TABLE IF NOT EXISTS events (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    start_date DATETIME,
    is_active BOOLEAN NOT NULL DEFAULT 1,
    score REAL,
    metadata JSON
);

INSERT INTO tables (number) VALUES (1), (2), (3), (4), (5);

INSERT INTO items (food_type, time_takes_to_cook, table_id, points)
VALUES
  ('korean', 10, 1, 2),
  ('chinese', 10, 1, 2),
  ('japanese', 10, 1, 2),
  ('italian', 10, 1, 2),
  ('french', 10, 1, 2);

INSERT INTO events (name, description, start_date, is_active, score)
VALUES
  ('Lunch Special', 'Daily lunch menu', '2024-01-15 12:00:00', 1, 4.5),
  ('Happy Hour', NULL, '2024-01-15 17:00:00', 1, 3.8);
