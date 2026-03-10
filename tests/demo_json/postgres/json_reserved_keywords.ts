import { sql } from 'sqlx-ts'

// Test jsonb_build_object with TypeScript reserved keywords
const jsonbReservedKeywords = sql`
-- @name: jsonb reserved keywords
SELECT
  json_test_data.id,
  json_test_data.name,
  jsonb_build_object(
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

// Test jsonb_build_object with invalid TypeScript identifiers
const jsonbInvalidIdentifiers = sql`
-- @name: jsonb invalid identifiers
SELECT
  json_test_data.id,
  json_test_data.name,
  jsonb_build_object(
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

// Test jsonb_agg with reserved keywords
const jsonbAggReservedKeywords = sql`
-- @name: jsonb agg reserved keywords
SELECT
  jsonb_agg(
    jsonb_build_object(
      'class', json_test_data.name,
      'interface', json_test_data.id,
      'default', true
    )
  ) AS aggregated_reserved_keywords
FROM json_test_data
LIMIT 3
`

// Test mixed valid and invalid identifiers
const jsonbMixedIdentifiers = sql`
-- @name: jsonb mixed identifiers
SELECT
  json_test_data.id,
  jsonb_build_object(
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
