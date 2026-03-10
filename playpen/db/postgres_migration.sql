DROP SCHEMA public CASCADE;
CREATE SCHEMA public;

CREATE EXTENSION IF NOT EXISTS pgcrypto;

GRANT ALL ON SCHEMA public TO postgres;
GRANT ALL ON SCHEMA public TO public;

CREATE TYPE faction_enum AS ENUM (
  'alliance',
  'horde'
);

-- Factions Table
CREATE TABLE factions (
  id SERIAL PRIMARY KEY,
  name faction_enum UNIQUE NOT NULL,
  description TEXT
);

-- Create the races table with an enum for the race names
CREATE TYPE race_enum AS ENUM (
  'human',
  'night elf',
  'dwarf',
  'gnome',
  'orc',
  'troll',
  'tauren',
  'undead'
);

-- Races Table
CREATE TABLE races (
  id SERIAL PRIMARY KEY,
  name race_enum UNIQUE NOT NULL,
  faction_id INTEGER REFERENCES factions(id) ON DELETE CASCADE
);

-- Create the races table with an enum for the race names
CREATE TYPE class_enum AS ENUM (
  'warrior',
  'hunter',
  'priest',
  'paladin',
  'druid',
  'mage',
  'warlock'
);

-- Classes Table
CREATE TABLE classes (
  id SERIAL PRIMARY KEY,
  name class_enum UNIQUE NOT NULL,
  specialization JSONB DEFAULT '{}',

  -- JSON Schema Validation (CHECK constraint)
    CHECK (
      jsonb_typeof(specialization) = 'object' AND
      (specialization ? 'role') AND
      (specialization ? 'weapon') AND
      (specialization ? 'abilities') AND

      -- Validate role with enum-like restriction
      specialization->>'role' IN ('tank', 'healer', 'ranged', 'melee', 'hybrid') AND

      -- Ensure abilities is an array with at least one element
      jsonb_typeof(specialization->'abilities') = 'array' AND
      jsonb_array_length(specialization->'abilities') > 0 AND

      -- Validate tier (if it exists) is between 1 and 5
      (
        NOT (specialization ? 'tier') OR
        (
          (specialization->>'tier')::integer BETWEEN 1 AND 5
        )
      )
    )
);

-- Characters Table
CREATE TABLE characters (
  id SERIAL PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  race_id INTEGER REFERENCES races(id),
  class_id INTEGER REFERENCES classes(id),
  level SMALLINT DEFAULT 1,
  experience BIGINT DEFAULT 0,
  gold FLOAT8 DEFAULT 0,
  last_chat_time TIME null,
  login_time TIMESTAMP null,
  logout_time TIMESTAMPTZ null,
  last_trade_time interval,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Guilds Table
CREATE TABLE guilds (
  id SERIAL PRIMARY KEY,
  name VARCHAR(100) UNIQUE NOT NULL,
  description TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Guild Members Table
CREATE TABLE guild_members (
  guild_id INTEGER REFERENCES guilds(id) ON DELETE CASCADE,
  character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
  rank VARCHAR(50),
  joined_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (guild_id, character_id)
);

-- Inventory Table
CREATE TABLE inventory (
  id SERIAL PRIMARY KEY,
  character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
  quantity INTEGER DEFAULT 1
);

-- Items Table
CREATE TABLE items (
  id SERIAL PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  rarity VARCHAR(50),
  flavor_text TEXT,
  inventory_id INTEGER REFERENCES inventory(id) ON DELETE CASCADE
);

-- Quests Table
CREATE TABLE quests (
  id SERIAL PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  description TEXT,
  rewards JSONB DEFAULT '{}',
  completed BOOLEAN DEFAULT false,
  required_level INTEGER DEFAULT 1
);

-- Character Quests Table
CREATE TABLE character_quests (
  character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
  quest_id INTEGER REFERENCES quests(id) ON DELETE CASCADE,
  status VARCHAR(50) DEFAULT 'In Progress',
  PRIMARY KEY (character_id, quest_id)
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

	enum1 faction_enum,

    -- Special data types
   	box1 BOX,
   	point1 POINT,
   	lseg1 LSEG,
   	polygon1 POLYGON,
   	inet1 INET,
   	macaddr1 MACADDR
);

--- SEED DATA

INSERT INTO factions (name, description) VALUES
('alliance', 'The noble and righteous faction'),
('horde', 'The fierce and battle-hardened faction');

INSERT INTO races (name, faction_id) VALUES
('human', 1),
('night elf', 1),
('dwarf', 1),
('gnome', 1),
('orc', 2),
('troll', 2),
('tauren', 2),
('undead', 2);

INSERT INTO classes (name, specialization) VALUES
('warrior', '{"role": "tank", "weapon": "sword", "abilities": ["charge", "slam", "shield block"]}'),
('hunter', '{"role": "ranged", "weapon": "bow", "abilities": ["aimed shot", "multi-shot", "trap"]}'),
('priest', '{"role": "healer", "weapon": "staff", "abilities": ["heal", "shield", "resurrect"]}'),
('paladin', '{"role": "tank", "weapon": "mace", "abilities": ["divine shield", "hammer of justice", "consecrate"]}'),
('druid', '{"role": "hybrid", "weapon": "staff", "abilities": ["shapeshift", "moonfire", "regrowth"]}'),
('mage', '{"role": "ranged", "weapon": "wand", "abilities": ["fireball", "frostbolt", "arcane blast"]}'),
('warlock', '{"role": "ranged", "weapon": "dagger", "abilities": ["summon demon", "shadowbolt", "curse of agony"]}');

-- JSON Test Data Table
-- This table contains various JSON structures for testing JSON operators and functions
CREATE TABLE json_test_data (
  id SERIAL PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  data JSONB NOT NULL,
  metadata JSON,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO json_test_data (name, data, metadata) VALUES
-- Simple object
('user_profile',
 '{"userId": 1, "username": "john_doe", "email": "john@example.com", "age": 30, "active": true}',
 '{"source": "api", "version": "1.0"}'),

-- Nested object with address
('user_with_address',
 '{"userId": 2, "username": "jane_smith", "email": "jane@example.com", "address": {"street": "123 Main St", "city": "Springfield", "state": "IL", "zipCode": "62701", "country": "USA"}}',
 '{"source": "import", "version": "1.0"}'),

-- Array of items
('shopping_cart',
 '{"cartId": 101, "items": [{"productId": 1, "name": "Laptop", "quantity": 1, "price": 999.99}, {"productId": 2, "name": "Mouse", "quantity": 2, "price": 25.50}], "totalPrice": 1050.99}',
 '{"source": "web", "version": "2.0"}'),

-- Array of strings
('tags',
 '{"postId": 42, "title": "PostgreSQL JSON Functions", "tags": ["database", "postgresql", "json", "tutorial"], "published": true}',
 '{"source": "cms", "version": "1.0"}'),

-- Nested arrays and objects
('game_stats',
 '{"playerId": 123, "stats": {"level": 50, "experience": 125000, "inventory": [{"slot": 1, "item": "Sword of Fire", "rarity": "legendary"}, {"slot": 2, "item": "Shield of Light", "rarity": "epic"}], "achievements": ["First Kill", "Level 50", "Legendary Item"]}}',
 '{"source": "game_server", "version": "3.0"}'),

-- Deep nesting
('nested_config',
 '{"app": {"name": "MyApp", "version": "1.0.0", "settings": {"database": {"host": "localhost", "port": 5432, "credentials": {"username": "admin", "encrypted": true}}, "features": {"darkMode": true, "notifications": {"email": true, "push": false}}}}}',
 '{"source": "config", "version": "1.0"}'),

-- Array of objects with nulls
('product_reviews',
 '{"productId": 456, "reviews": [{"reviewId": 1, "rating": 5, "comment": "Excellent product!", "reviewer": "Alice"}, {"reviewId": 2, "rating": 4, "comment": null, "reviewer": "Bob"}, {"reviewId": 3, "rating": 3, "comment": "Average", "reviewer": null}]}',
 '{"source": "reviews", "version": "1.0"}'),

-- Mixed types
('analytics',
 '{"date": "2024-01-15", "metrics": {"visitors": 1500, "pageViews": 4500, "bounceRate": 0.35, "sources": {"organic": 850, "direct": 400, "referral": 250}}}',
 '{"source": "analytics", "version": "1.0"}');
