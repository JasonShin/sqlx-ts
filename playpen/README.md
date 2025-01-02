# World of Warcraft Inspired SQL Schema

<img src="./assets/schema.png" width="80%">

This repository contains a comprehensive World of Warcraft-themed SQL schema designed to demonstrate the capabilities of [sqlx-ts](https://github.com/JasonShin/sqlx-ts). The schema models core game mechanics such as characters, races, classes, guilds, items, and quests, while incorporating PostgreSQL's advanced features like JSONB, ENUM types, and relationships.

## Overview
The schema covers:
- **Factions**: Represents the two primary factions - Alliance and Horde.
- **Races**: Lists all playable races, associated with their faction.
- **Classes**: Defines class specializations, including roles, weapons, and abilities stored as JSONB.
- **Characters**: Central entity representing player characters, linked to races and classes.
- **Guilds**: Guilds that characters can join, supporting a many-to-many relationship.
- **Items**: Collectible items with dynamic stats stored as JSONB.
- **Inventory**: Tracks character inventory, supporting multiple item quantities.
- **Quests**: Represents character progression, including quest rewards and status.
- **Random Data**: Demonstrates PostgreSQL's wide range of data types for testing and exploration.

## ERD Diagram
![ERD Diagram](./erd_diagram.png)

## Installation and Setup
1. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/sqlx-ts-wow-demo.git
   ```
2. Navigate to the project directory:
   ```bash
   cd sqlx-ts-wow-demo
   ```
3. Run the SQL schema on your PostgreSQL instance:
   ```bash
   psql -U yourusername -d yourdatabase -f schema.sql
   ```

## Key Features
- **Enum Types**: Faction, Race, and Class columns leverage PostgreSQL ENUMs for better data consistency.
- **JSONB Fields**: Flexible and scalable fields for class specializations, item stats, and quest rewards.
- **Referential Integrity**: Foreign key relationships ensure data consistency across races, classes, and factions.
- **Complex Constraints**: Advanced CHECK constraints validate JSONB fields to ensure proper specialization structures.

## Example Queries
Fetch all characters with their race and class:
```sql
SELECT c.name AS character_name, r.name AS race, cl.name AS class
FROM characters c
JOIN races r ON c.race_id = r.id
JOIN classes cl ON c.class_id = cl.id;
```

Retrieve all items in a character's inventory:
```sql
SELECT ch.name AS character_name, i.name AS item, inv.quantity
FROM inventory inv
JOIN characters ch ON inv.character_id = ch.id
JOIN items i ON inv.item_id = i.id;
```

List all active quests for a character:
```sql
SELECT q.name AS quest_name, cq.status
FROM character_quests cq
JOIN quests q ON cq.quest_id = q.id
WHERE cq.character_id = 1;
```

## License
This project is licensed under the MIT License. Feel free to modify and expand upon this schema for your own projects.

---
For any questions or suggestions, feel free to submit an issue or pull request!

