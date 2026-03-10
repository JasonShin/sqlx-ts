import { sql } from 'sqlx-ts'

// Test JSON_OBJECT with TypeScript reserved keywords
const jsonReservedKeywords = sql`
-- @db: db_mysql
-- @name: json reserved keywords
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  JSON_OBJECT(
    'class', 'User',
    'interface', 'IUser',
    'type', 'object',
    'const', 'constant',
    'let', 'variable',
    'function', 'method',
    'return', true,
    'import', 'module',
    'export', 'default',
    'async', 'promise'
  ) AS reserved_keywords_object
FROM json_test_data
WHERE json_test_data.name = 'user_profile'
LIMIT 1
`

// Test JSON_OBJECT with invalid TypeScript identifiers
const jsonInvalidIdentifiers = sql`
-- @db: db_mysql
-- @name: json invalid identifiers
SELECT
  json_test_data.id AS id,
  json_test_data.name AS name,
  JSON_OBJECT(
    'field-name', 'hyphenated',
    'field name', 'with space',
    '123field', 'starts with number',
    'user@email', 'special chars',
    'field.nested', 'dotted name'
  ) AS invalid_identifiers_object
FROM json_test_data
WHERE json_test_data.name = 'user_profile'
LIMIT 1
`

// Test mixed valid and invalid identifiers
const jsonMixedIdentifiers = sql`
-- @db: db_mysql
-- @name: json mixed identifiers
SELECT
  json_test_data.id AS id,
  JSON_OBJECT(
    'validName', json_test_data.name,
    'invalid-name', json_test_data.id,
    '_underscore', 'valid',
    '$dollar', 'valid',
    'class', 'reserved',
    '123start', 'invalid'
  ) AS mixed_identifiers_object
FROM json_test_data
WHERE json_test_data.name = 'user_profile'
LIMIT 1
`
