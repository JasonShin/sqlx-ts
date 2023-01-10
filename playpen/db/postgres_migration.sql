DROP SCHEMA public CASCADE;
CREATE SCHEMA public;

GRANT ALL ON SCHEMA public TO postgres;
GRANT ALL ON SCHEMA public TO public;

CREATE TABLE postgres.public.tables (
    id SERIAL NOT NULL,
    number INTEGER NOT NULL,
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

INSERT INTO public.tables (number) VALUES
(1), (2), (3), (4), (5), (6), (7), (8), (9), (10);

INSERT INTO public.items (food_type, time_takes_to_cook, table_id, points)
VALUES
  ('korean', 10, 1, 2),
  ('chinese', 10, 1, 2),
  ('japanese', 10, 1, 2),
  ('italian', 10, 1, 2),
  ('french', 10, 1, 2);
