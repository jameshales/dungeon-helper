#!/bin/bash

mkdir -p $(dirname "$DATABASE_PATH")

if [ ! -f "$DATABASE_PATH" ]; then
  db_path=$(mktemp -d)
  cat /opt/dungeon-helper/share/sql/*.sql | sqlite3 $db_path/dungeon-helper.db
  mv -n $db_path/dungeon-helper.db "$DATABASE_PATH"
  rmdir $db_path
fi

cd /opt/dungeon-helper/var/

exec /opt/dungeon-helper/bin/dungeon_helper
