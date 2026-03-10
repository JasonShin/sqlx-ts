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

-- Inventory Table
CREATE TABLE inventory (
  id INT AUTO_INCREMENT PRIMARY KEY,
  quantity INT DEFAULT 1,
  character_id INT,
  FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE
);

-- Items Table
CREATE TABLE items (
  id INT AUTO_INCREMENT PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  rarity VARCHAR(50),
  inventory_id INT,
  flavor_text TEXT,
  FOREIGN KEY (inventory_id) REFERENCES inventory(id) ON DELETE CASCADE
);

-- Quests Table
CREATE TABLE quests (
  id INT AUTO_INCREMENT PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  description TEXT,
  rewards JSON,
  completed BOOL DEFAULT false,
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
	enum1 ENUM('alliance', 'horde'),
	set1 SET('one', 'two'),

	-- JSON types
	json1 JSON
);

-- JSON Test Data Table
-- This table contains various JSON structures for testing JSON operators and functions
CREATE TABLE json_test_data (
  id INT AUTO_INCREMENT PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  data JSON NOT NULL,
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
 '{"postId": 42, "title": "MySQL JSON Functions", "tags": ["database", "mysql", "json", "tutorial"], "published": true}',
 '{"source": "cms", "version": "1.0"}'),

-- Nested arrays and objects
('game_stats',
 '{"playerId": 123, "stats": {"level": 50, "experience": 125000, "inventory": [{"slot": 1, "item": "Sword of Fire", "rarity": "legendary"}, {"slot": 2, "item": "Shield of Light", "rarity": "epic"}], "achievements": ["First Kill", "Level 50", "Legendary Item"]}}',
 '{"source": "game_server", "version": "3.0"}'),

-- Deep nesting
('nested_config',
 '{"app": {"name": "MyApp", "version": "1.0.0", "settings": {"database": {"host": "localhost", "port": 3306, "credentials": {"username": "admin", "encrypted": true}}, "features": {"darkMode": true, "notifications": {"email": true, "push": false}}}}}',
 '{"source": "config", "version": "1.0"}'),

-- Array of objects with nulls
('product_reviews',
 '{"productId": 456, "reviews": [{"reviewId": 1, "rating": 5, "comment": "Excellent product!", "reviewer": "Alice"}, {"reviewId": 2, "rating": 4, "comment": null, "reviewer": "Bob"}, {"reviewId": 3, "rating": 3, "comment": "Average", "reviewer": null}]}',
 '{"source": "reviews", "version": "1.0"}'),

-- Mixed types
('analytics',
 '{"date": "2024-01-15", "metrics": {"visitors": 1500, "pageViews": 4500, "bounceRate": 0.35, "sources": {"organic": 850, "direct": 400, "referral": 250}}}',
 '{"source": "analytics", "version": "1.0"}');
