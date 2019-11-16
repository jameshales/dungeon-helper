CREATE TABLE weapons (name TEXT PRIMARY KEY);

INSERT INTO weapons (name) VALUES ('Battleaxe');
INSERT INTO weapons (name) VALUES ('Club');
INSERT INTO weapons (name) VALUES ('Hand Crossbow');
INSERT INTO weapons (name) VALUES ('Heavy Crossbow');
INSERT INTO weapons (name) VALUES ('Light Crossbow');
INSERT INTO weapons (name) VALUES ('Dagger');
INSERT INTO weapons (name) VALUES ('Dart');
INSERT INTO weapons (name) VALUES ('Flail');
INSERT INTO weapons (name) VALUES ('Glaive');
INSERT INTO weapons (name) VALUES ('Greataxe');
INSERT INTO weapons (name) VALUES ('Greatclub');
INSERT INTO weapons (name) VALUES ('Greatsword');
INSERT INTO weapons (name) VALUES ('Halberd');
INSERT INTO weapons (name) VALUES ('Handaxe');
INSERT INTO weapons (name) VALUES ('Javelin');
INSERT INTO weapons (name) VALUES ('Lance');
INSERT INTO weapons (name) VALUES ('Light Hammer');
INSERT INTO weapons (name) VALUES ('Longbow');
INSERT INTO weapons (name) VALUES ('Longsword');
INSERT INTO weapons (name) VALUES ('Mace');
INSERT INTO weapons (name) VALUES ('Maul');
INSERT INTO weapons (name) VALUES ('Morningstar');
INSERT INTO weapons (name) VALUES ('Pike');
INSERT INTO weapons (name) VALUES ('Quarterstaff');
INSERT INTO weapons (name) VALUES ('Rapier');
INSERT INTO weapons (name) VALUES ('Scimitar');
INSERT INTO weapons (name) VALUES ('Shortbow');
INSERT INTO weapons (name) VALUES ('Shortsword');
INSERT INTO weapons (name) VALUES ('Sickle');
INSERT INTO weapons (name) VALUES ('Sling');
INSERT INTO weapons (name) VALUES ('Spear');
INSERT INTO weapons (name) VALUES ('Trident');
INSERT INTO weapons (name) VALUES ('War Pick');
INSERT INTO weapons (name) VALUES ('Warhammer');
INSERT INTO weapons (name) VALUES ('Whip');

CREATE TABLE character_weapon_proficiencies (
  channel_id TEXT NOT NULL,
  user_id TEXT NOT NULL,
  weapon_name TEXT NULL REFERENCES weapons (name),
  weapon_category TEXT NULL,
  FOREIGN KEY (channel_id, user_id) REFERENCES characters (channel_id, user_id),
  CHECK (weapon_category = 'Simple' OR weapon_category = 'Martial' or weapon_category is null),
  CHECK ((weapon_name is null) <> ( weapon_category is null)),
  UNIQUE (channel_id, user_id, weapon_name),
  UNIQUE (channel_id, user_id, weapon_category)
);
