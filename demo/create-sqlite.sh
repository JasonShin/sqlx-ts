#!/bin/bash
# Creates the SQLite demo database from setup.sql
set -e
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
DB_PATH="$SCRIPT_DIR/sqlite/demo.db"

rm -f "$DB_PATH"
sqlite3 "$DB_PATH" < "$SCRIPT_DIR/setup.sql"
echo "Created SQLite database at $DB_PATH"
