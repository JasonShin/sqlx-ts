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

Alternatively, you can use `DB_URL` to specify the connection string directly:

```json
{
  "generate_types": {
    "enabled": true
  },
  "connections": {
    "default": {
      "DB_TYPE": "postgres",
      "DB_URL": "postgres://postgres:postgres@127.0.0.1:5432/mydb"
    },
    "mysql_db": {
      "DB_TYPE": "mysql",
      "DB_URL": "mysql://root:password@127.0.0.1:3306/mydatabase"
    }
  }
}
```

For SQLite, only `DB_TYPE` and `DB_NAME` (the file path) are required:

```json
{
  "generate_types": {
    "enabled": true
  },
  "connections": {
    "default": {
      "DB_TYPE": "sqlite",
      "DB_NAME": "./mydb.sqlite"
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
- `DB_URL`: Database connection URL (e.g. `postgres://user:pass@host:port/dbname` or `mysql://user:pass@host:port/dbname`). If provided, this overrides individual connection parameters (`DB_HOST`, `DB_PORT`, `DB_USER`, `DB_PASS`, `DB_NAME`)
- `DB_TYPE`: type of database connection (mysql | postgres | sqlite)
- `DB_USER`: database user name
- `DB_PASS`: database password
- `DB_HOST`: database host (e.g. 127.0.0.1)
- `DB_PORT`: database port (e.g. 4321)
- `PG_SEARCH_PATH`: PostgreSQL schema search path (default is "$user,public") [https://www.postgresql.org/docs/current/ddl-schemas.html#DDL-SCHEMAS-PATH](https://www.postgresql.org/docs/current/ddl-schemas.html#DDL-SCHEMAS-PATH)
- `POOL_SIZE`: Size of the connection pool to establish per connection type
- `CONNECTION_TIMEOUT`: Timeout in second of Database connection attempt
- `type_mapping`: Overrides the generated TypeScript type of database column types (see below)

#### type_mapping

By default, SQLX-TS translates each database column type into a built-in TypeScript type (e.g. `bigint` -> `number`).
You can override this per connection by mapping a database column type to any TypeScript type. A mapping is either
a type, or an object with `type` and an optional `import`; the import statement is added at the top of the generated
types file.

```json
{
  "connections": {
    "default": {
      "DB_TYPE": "postgres",
      "DB_URL": "postgres://postgres:postgres@127.0.0.1:5432/mydb",
      "type_mapping": {
        "bigint": "string",
        "numeric": "string | number",
        "_int8": "string[]",
        "timestamp": { "type": "DateTime", "import": "import type { DateTime } from 'luxon'" }
      }
    }
  }
}
```

**Matching**

- Keys are matched case-insensitively against the column type reported by the database, e.g. `bigint`, `DATETIME`
- An exact match wins, otherwise type modifiers are ignored, so `varchar` matches `VARCHAR(255)` and `bigint unsigned` matches `bigint(20) unsigned`
- PostgreSQL: both the `data_type` (e.g. `timestamp without time zone`) and the `udt_name` (e.g. `timestamp`, `int8`, or the name of an enum or extension type such as `citext`) are matched. Domain types are matched by their underlying type
- PostgreSQL arrays: array columns are reported as `ARRAY`, so the mapping of the element type does not apply. Map the array's `udt_name` instead, which is the element type prefixed with `_`, e.g. `"_int8": "string[]"`
- MySQL: both the full `COLUMN_TYPE` (e.g. `tinyint(1)`, `bigint unsigned`) and the `DATA_TYPE` (e.g. `tinyint`, `bigint`) are matched, with `COLUMN_TYPE` taking priority. This lets you map `"tinyint(1)": "boolean"` separately from `"tinyint": "number"`

**Generated types**

- Nullable columns still produce `| null`, e.g. `string | null`
- The mapping applies to both query results and parameters
- `@result` and `@param` [annotations](../type-generation/annotations.md) take priority over the mapping
- The mapping only applies to values typed from a table column, including expressions derived from one such as `COALESCE(col, 0)`. Types from the SQL itself, such as `CAST(col AS BIGINT)` or column definitions of a table-valued function (`jsonb_to_recordset($1) AS t(id BIGINT)`), are not mapped

**Imports**

- Each import is written once per generated file, including when all types are generated into a single file with `--generate-path`
- Multi-line imports are written on a single line
- Imports are de-duplicated by their text, so use the same import statement for every mapping that imports the same type. For example, `import type { DateTime } from 'luxon'` and `import type { DateTime } from "luxon"` are written as two imports, which TypeScript reports as a duplicate identifier

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
