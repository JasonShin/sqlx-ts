# sqlx-ts demo

Usage examples showing sqlx-ts with different database drivers.

## Examples

- [pg](./pg) - PostgreSQL with `pg` driver
- [mysql2](./mysql2) - MySQL with `mysql2` driver
- [sequelize](./sequelize) - PostgreSQL with Sequelize ORM
- [sqlite](./sqlite) - SQLite with `better-sqlite3` (no Docker needed)

## Running locally

```bash
# Start databases (from repo root)
docker compose up -d

# Install demo dependencies
cd demo
npm install

# Generate types and type-check
npm run compile:all
npm run typecheck
```
