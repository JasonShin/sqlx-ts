---
sidebar_position: 3
---

# Connecting to databases

`sqlx-ts` supports the following approach connect to the database

1. [File based configuration](./1.3.configs-file-based.md)
2. [CLI options](#2-cli-options)
3. [Environment variables](./1.2.environment-variables.md)

If you provide database host by a CLI option and an environment variable, CLI option will take
the priority over the environment variable.

### 1. File based config

If your project needs connections to multiple databases for SQL check, you have to use the file
based config and specify connection details for those databases. [Please check here for more details](./1.3.configs-file-based.md)

### 2. CLI options

Run the following command for more details

```bash
$ sqlx-ts --help
```

### 3. Environment variables

You can only configure the primary database connection through environment variables. [Please check here for more details](./1.2.environment-variables.md)
