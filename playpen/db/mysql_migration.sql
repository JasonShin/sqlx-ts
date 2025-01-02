CREATE TABLE faction_enum (
  name ENUM('alliance', 'horde') NOT NULL PRIMARY KEY
);

CREATE TABLE race_enum (
  name ENUM('human', 'night elf', 'dwarf', 'gnome', 'orc', 'troll', 'tauren', 'undead') NOT NULL PRIMARY KEY
);

CREATE TABLE class_enum (
  name ENUM('warrior', 'hunter', 'priest', 'paladin', 'druid', 'mage', 'warlock') NOT NULL PRIMARY KEY
);

-- Factions Table
CREATE TABLE factions (
  id INT AUTO_INCREMENT PRIMARY KEY,
  name ENUM('alliance', 'horde') UNIQUE NOT NULL,
  description TEXT
);

-- Races Table
CREATE TABLE races (
  id INT AUTO_INCREMENT PRIMARY KEY,
  name ENUM('human', 'night elf', 'dwarf', 'gnome', 'orc', 'troll', 'tauren', 'undead') UNIQUE NOT NULL,
  faction_id INT,
  FOREIGN KEY (faction_id) REFERENCES factions(id) ON DELETE CASCADE
);

-- Classes Table
CREATE TABLE classes (
  id INT AUTO_INCREMENT PRIMARY KEY,
  name ENUM('warrior', 'hunter', 'priest', 'paladin', 'druid', 'mage', 'warlock') UNIQUE NOT NULL,
  specialization JSON,
  CHECK (
    JSON_VALID(specialization) AND
    JSON_UNQUOTE(JSON_EXTRACT(specialization, '$.role')) IN ('tank', 'healer', 'ranged', 'melee', 'hybrid') AND
    JSON_LENGTH(JSON_EXTRACT(specialization, '$.abilities')) > 0 AND
    (
      JSON_EXTRACT(specialization, '$.tier') IS NULL OR
      CAST(JSON_UNQUOTE(JSON_EXTRACT(specialization, '$.tier')) AS UNSIGNED) BETWEEN 1 AND 5
    )
  )
);

-- Characters Table
CREATE TABLE characters (
  id INT AUTO_INCREMENT PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  race_id INT,
  class_id INT,
  level SMALLINT DEFAULT 1,
  experience BIGINT DEFAULT 0,
  gold DOUBLE DEFAULT 0,
  last_chat_time TIME,
  login_time DATETIME,
  logout_time DATETIME,
  last_trade_time TIME,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (race_id) REFERENCES races(id),
  FOREIGN KEY (class_id) REFERENCES classes(id)
);

-- Guilds Table
CREATE TABLE guilds (
  id INT AUTO_INCREMENT PRIMARY KEY,
  name VARCHAR(100) UNIQUE NOT NULL,
  description TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Guild Members Table
CREATE TABLE guild_members (
  guild_id INT,
  character_id INT,
  guild_rank VARCHAR(50),
  joined_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (guild_id, character_id),
  FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
  FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE
);

-- Items Table
CREATE TABLE items (
  id INT AUTO_INCREMENT PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  rarity VARCHAR(50),
  stats JSON,
  flavor_text TEXT
);

-- Inventory Table
CREATE TABLE inventory (
  character_id INT,
  item_id INT,
  quantity INT DEFAULT 1,
  PRIMARY KEY (character_id, item_id),
  FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE,
  FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE CASCADE
);

-- Quests Table
CREATE TABLE quests (
  id INT AUTO_INCREMENT PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  description TEXT,
  rewards JSON,
  required_level INT DEFAULT 1
);

-- Character Quests Table
CREATE TABLE character_quests (
  character_id INT,
  quest_id INT,
  status VARCHAR(50) DEFAULT 'In Progress',
  PRIMARY KEY (character_id, quest_id),
  FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE,
  FOREIGN KEY (quest_id) REFERENCES quests(id) ON DELETE CASCADE
);

-- Seed Data
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
