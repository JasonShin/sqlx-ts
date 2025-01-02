DROP SCHEMA public CASCADE;
CREATE SCHEMA public;

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

-- Items Table
CREATE TABLE items (
  id SERIAL PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  rarity VARCHAR(50),
  stats JSONB DEFAULT '{}',
  flavor_text TEXT
);

-- Inventory Table
CREATE TABLE inventory (
  character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
  item_id INTEGER REFERENCES items(id) ON DELETE CASCADE,
  quantity INTEGER DEFAULT 1,
  PRIMARY KEY (character_id, item_id)
);

-- Quests Table
CREATE TABLE quests (
  id SERIAL PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  description TEXT,
  rewards JSONB DEFAULT '{}',
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
