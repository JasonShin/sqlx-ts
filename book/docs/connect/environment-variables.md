# Environment variables


| Environment variables | Description                                                                                |
| --------------------- | ------------------------------------------------------------------------------------------ |
| DB_URL                | Primary database connection URL (e.g. `postgres://user:pass@host:port/dbname` or `mysql://user:pass@host:port/dbname`). If provided, this overrides individual connection parameters |
| DB_HOST               | Primary DB host                                                                            |
| DB_PASS               | Primary DB password                                                                        |
| DB_PORT               | Primary DB port number                                                                     |
| DB_TYPE               | Type of primary database to connect [default: postgres] [possible values: postgres, mysql] |
| DB_USER               | Primary DB user name                                                                       |
| DB_NAME               | Primary DB name                                                                            |
| PG_SEARCH_PATH        | PostgreSQL schema search path (default is "$user,public") [https://www.postgresql.org/docs/current/ddl-schemas.html#DDL-SCHEMAS-PATH](https://www.postgresql.org/docs/current/ddl-schemas.html#DDL-SCHEMAS-PATH)                                                                                           |

## Examples

### Using individual connection parameters

```bash
export DB_TYPE=postgres
export DB_HOST=127.0.0.1
export DB_PORT=5432
export DB_USER=postgres
export DB_PASS=postgres
export DB_NAME=mydb

sqlx-ts <path>
```

### Using database URL

Alternatively, you can use `DB_URL` to specify the complete connection string:

```bash
# PostgreSQL
export DB_TYPE=postgres
export DB_URL=postgres://postgres:postgres@localhost:5432/mydb

sqlx-ts <path>

# MySQL
export DB_TYPE=mysql
export DB_URL=mysql://root:password@localhost:3306/mydatabase

sqlx-ts <path>
```

**Note:** When `DB_URL` is set, it takes precedence over individual connection parameters (`DB_HOST`, `DB_PORT`, `DB_USER`, `DB_PASS`, `DB_NAME`).

