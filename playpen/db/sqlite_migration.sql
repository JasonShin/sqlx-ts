-- SQLite Migration
-- This migration creates the same tables as the PostgreSQL and MySQL migrations
-- but using SQLite-compatible syntax.

-- Factions Table
CREATE TABLE IF NOT EXISTS factions (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT UNIQUE NOT NULL,
  description TEXT
);

-- Races Table
CREATE TABLE IF NOT EXISTS races (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT UNIQUE NOT NULL,
  faction_id INTEGER REFERENCES factions(id) ON DELETE CASCADE
);

-- Classes Table
CREATE TABLE IF NOT EXISTS classes (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT UNIQUE NOT NULL,
  specialization TEXT
);

-- Characters Table
CREATE TABLE IF NOT EXISTS characters (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  race_id INTEGER REFERENCES races(id),
  class_id INTEGER REFERENCES classes(id),
  level INTEGER DEFAULT 1,
  experience INTEGER DEFAULT 0,
  gold REAL DEFAULT 0,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Guilds Table
CREATE TABLE IF NOT EXISTS guilds (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT UNIQUE NOT NULL,
  description TEXT,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Guild Members Table
CREATE TABLE IF NOT EXISTS guild_members (
  guild_id INTEGER REFERENCES guilds(id) ON DELETE CASCADE,
  character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
  rank TEXT,
  joined_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (guild_id, character_id)
);

-- Inventory Table
CREATE TABLE IF NOT EXISTS inventory (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
  quantity INTEGER DEFAULT 1
);

-- Items Table
CREATE TABLE IF NOT EXISTS items (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  rarity TEXT,
  flavor_text TEXT,
  inventory_id INTEGER REFERENCES inventory(id) ON DELETE CASCADE
);

-- Quests Table
CREATE TABLE IF NOT EXISTS quests (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  description TEXT,
  rewards TEXT,
  completed BOOLEAN DEFAULT 0,
  required_level INTEGER DEFAULT 1
);

-- Character Quests Table
CREATE TABLE IF NOT EXISTS character_quests (
  character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
  quest_id INTEGER REFERENCES quests(id) ON DELETE CASCADE,
  status TEXT DEFAULT 'In Progress',
  PRIMARY KEY (character_id, quest_id)
);

-- Random types table for testing SQLite type mappings
CREATE TABLE IF NOT EXISTS random (
  int1 INTEGER,
  real1 REAL,
  text1 TEXT,
  blob1 BLOB,
  numeric1 NUMERIC,
  bool1 BOOLEAN,
  date1 DATE,
  datetime1 DATETIME,
  float1 FLOAT,
  double1 DOUBLE,
  varchar1 VARCHAR(100),
  char1 CHAR(10),
  json1 JSON
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
('hunter', '{"role": "ranged", "weapon": "bow", "abilities": ["aimed shot", "multi-shot", "trap"]}');
