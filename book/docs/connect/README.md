---
sidebar_position: 3
---

# Connecting to databases

`sqlx-ts` supports the following approach connect to the database

1. [File based configuration](/connect/config-file)
2. [CLI options](#2-cli-options)
3. [Environment variables](/connect/environment-variables)

If you provide database host by a CLI option and an environment variable, CLI option will take
the priority over the environment variable.

### 1. File based config

If your project needs connections to multiple databases for SQL check, you have to use the file
based config and specify connection details for those databases. [Please check here for more details](/connect/config-file)

### 2. CLI options

You can provide database connection details via CLI options. This includes individual connection parameters or a complete database URL.

#### Using individual connection parameters

```bash
$ sqlx-ts <path> \
  --db-type postgres \
  --db-host 127.0.0.1 \
  --db-port 5432 \
  --db-user postgres \
  --db-pass postgres \
  --db-name mydb
```

#### Using database URL

Alternatively, you can use `--db-url` to specify the complete connection string:

```bash
# PostgreSQL
$ sqlx-ts <path> --db-type postgres --db-url postgres://user:pass@localhost:5432/mydb

# MySQL
$ sqlx-ts <path> --db-type mysql --db-url mysql://user:pass@localhost:3306/mydb
```

**Note:** When `--db-url` is provided, it takes precedence over individual connection parameters (`--db-host`, `--db-port`, `--db-user`, `--db-pass`, `--db-name`).

Run the following command for more details:

```bash
$ sqlx-ts --help
```

### 3. Environment variables

You can only configure the primary database connection through environment variables. [Please check here for more details](/connect/environment-variables)
