#!/bin/bash

mkdir -p /opt/dungeon-helper/var/

if [ ! -f /opt/dungeon-helper/var/dungeon-helper.db ]; then
  db_path=$(mktemp -d)
  cat /opt/dungeon-helper/share/sql/*.sql | sqlite3 $db_path/dungeon-helper.db
  mv -n $db_path/dungeon-helper.db /opt/dungeon-helper/var/dungeon-helper.db
  rmdir $db_path
fi

cd /opt/dungeon-helper/var/

exec /opt/dungeon-helper/bin/dungeon_helper
