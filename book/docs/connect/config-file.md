# Configuration file

If you have a project that you need requires connections to multiple databases, you can support 
that by using file based configuration.

By default, configuration file is named `.sqlxrc.json` and SQLX-TS will try to find a file with 
this name, unless you give it a custom path to override it using `--config` CLI option.

```bash
$ sqlx-ts --config <path to a custom .sqlxrc.json>
```

Example `.sqlxrc.json`

```json
{
  "generate_types": {
    "enabled": true,
    "convertToCamelCaseColumnName": true
  },
  "connections": {
    "default": {
      "DB_TYPE": "mysql",
      "DB_USER": "root",
      "DB_HOST": "127.0.0.1",
      "DB_PORT": 3306
    },
    "postgres": {
      "DB_TYPE": "postgres",
      "DB_USER": "postgres",
      "DB_PASS": "postgres",
      "DB_HOST": "127.0.0.1",
      "DB_PORT": 4321,
      "PG_SEARCH_PATH": "public,myschema"
    },
    "some_other_db": {
      "DB_TYPE": "mysql",
      "DB_USER": "app_user",
      "DB_PASS": "password",
      "DB_HOST": "127.0.0.1",
      "DB_PORT": 3307,
      "POOL_SIZE": 20,
      "CONNECTION_TIMEOUT": 10
    }
  }
}
```

## Configuration options

### connections (required)

For default database, you must call it `default` like example above. Any extra DB connections 
should have its own unique name such as `postgres` or `some_other_db`

Along with the configuration above, when writing SQLs in your codebase, you need to provide 
supportive comment in your raw SQL, indicate which database the query should point.

For example,

```typescript
import { sql } from 'sqlx-ts'

// targets the default DB
const defaultDbSQL = sql`SELECT * FROM test;`
// targets the config with the name `postgres`
const postgresSQL = sql`
 -- @db: postgres
 SELECT * FROM other_table;
`
```

Supported fields of each connection include
- `DB_TYPE`: type of database connection (mysql | postgres)
- `DB_USER`: database user name
- `DB_PASS`: database password
- `DB_HOST`: database host (e.g. 127.0.0.1)
- `DB_PORT`: database port (e.g. 4321)
- `PG_SEARCH_PATH`: PostgreSQL schema search path (default is "$user,public") [https://www.postgresql.org/docs/current/ddl-schemas.html#DDL-SCHEMAS-PATH](https://www.postgresql.org/docs/current/ddl-schemas.html#DDL-SCHEMAS-PATH)
- `POOL_SIZE`: Size of the connection pool to establish per connection type
- `CONNECTION_TIMEOUT`: Timeout in second of Database connection attempt

### generate_types

```json
{
  "generateTypes: {
    enabled: true|false,
    columnNamingConvention: "upper | lower | title | camel | pascal | snake | kebab"
  },
  "connections": {
    ...
  }
}
```

Support for configuration of generate types operations.
- `enabled` (default: false): enables type generation via config
- `columnNamingConvention` (optional): When generating field name based on table's column name, you can pass in a type of naming convention to be used
  - oneOf: upper | lower | title | camel | pascal | snake | kebab
