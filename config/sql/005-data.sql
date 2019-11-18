-- LBD&D #dice
INSERT INTO channels (
  channel_id,
  enabled,
  locked,
  dice_only
) VALUES (
  '644028953719209984',
  true,
  true,
  true
);

-- DHD #dice
INSERT INTO channels (
  channel_id,
  enabled,
  locked,
  dice_only
) VALUES (
  '643662493557850129',
  true,
  false,
  true
);

-- Grem
INSERT INTO characters (
  channel_id,
  user_id,
  level,
  strength,
  dexterity,
  constitution,
  intelligence,
  wisdom,
  charisma,
  strength_saving_proficiency,
  constitution_saving_proficiency,
  animal_handling_proficiency,
  athletics_proficiency,
  intimidation_proficiency,
  nature_proficiency,
  survival_proficiency
) VALUES (
  '644028953719209984',
  '194800877960167424',
  5,
  18,
  13,
  16,
  8,
  10,
  12,
  true,
  true,
  'Proficient',
  'Proficient',
  'Proficient',
  'Proficient',
  'Proficient'
);

INSERT INTO character_weapon_proficiencies (
  channel_id,
  user_id,
  weapon_category
) VALUES (
  '644028953719209984',
  '194800877960167424',
  'Simple'
);

INSERT INTO character_weapon_proficiencies (
  channel_id,
  user_id,
  weapon_category
) VALUES (
  '644028953719209984',
  '194800877960167424',
  'Martial'
);

-- Roland
INSERT INTO characters (
  channel_id,
  user_id,
  level,
  strength,
  dexterity,
  constitution,
  intelligence,
  wisdom,
  charisma,
  wisdom_saving_proficiency,
  charisma_saving_proficiency,
  insight_proficiency,
  intimidation_proficiency,
  medicine_proficiency,
  religion_proficiency
) VALUES (
  '644028953719209984',
  '448364875299946506',
  5,
  17,
  9,
  14,
  11,
  13,
  16,
  true,
  true,
  'Proficient',
  'Proficient',
  'Proficient',
  'Proficient'
);

INSERT INTO character_weapon_proficiencies (
  channel_id,
  user_id,
  weapon_category
) VALUES (
  '644028953719209984',
  '448364875299946506',
  'Simple'
);

INSERT INTO character_weapon_proficiencies (
  channel_id,
  user_id,
  weapon_category
) VALUES (
  '644028953719209984',
  '448364875299946506',
  'Martial'
);

-- Laughter
INSERT INTO characters (
  channel_id,
  user_id,
  level,
  jack_of_all_trades,
  strength,
  dexterity,
  constitution,
  intelligence,
  wisdom,
  charisma,
  dexterity_saving_proficiency,
  charisma_saving_proficiency,
  acrobatics_proficiency,
  history_proficiency,
  insight_proficiency,
  intimidation_proficiency,
  nature_proficiency,
  performance_proficiency,
  persuasion_proficiency,
  sleight_of_hand_proficiency
) VALUES (
  '644028953719209984',
  '583587280296345601',
  5,
  true,
  10,
  14,
  13,
  9,
  12,
  19,
  true,
  true,
  'Proficient',
  'Proficient',
  'Proficient',
  'Expert',
  'Proficient',
  'Expert',
  'Proficient',
  'Proficient'
);

INSERT INTO character_weapon_proficiencies (
  channel_id,
  user_id,
  weapon_category
) VALUES (
  '644028953719209984',
  '583587280296345601',
  'Simple'
);

INSERT INTO character_weapon_proficiencies (
  channel_id,
  user_id,
  weapon_name
) VALUES (
  '644028953719209984',
  '583587280296345601',
  'Hand Crossbow'
);

INSERT INTO character_weapon_proficiencies (
  channel_id,
  user_id,
  weapon_name
) VALUES (
  '644028953719209984',
  '583587280296345601',
  'Longsword'
);

INSERT INTO character_weapon_proficiencies (
  channel_id,
  user_id,
  weapon_name
) VALUES (
  '644028953719209984',
  '583587280296345601',
  'Rapier'
);

INSERT INTO character_weapon_proficiencies (
  channel_id,
  user_id,
  weapon_name
) VALUES (
  '644028953719209984',
  '583587280296345601',
  'Shortsword'
);

-- Hardrum
INSERT INTO characters (
  channel_id,
  user_id,
  level,
  strength,
  dexterity,
  constitution,
  intelligence,
  wisdom,
  charisma,
  intelligence_saving_proficiency,
  wisdom_saving_proficiency,
  medicine_proficiency,
  nature_proficiency,
  religion_proficiency,
  survival_proficiency
) VALUES (
  '644028953719209984',
  '638138236874522634',
  5,
  14,
  8,
  15,
  16,
  15,
  10,
  true,
  true,
  'Proficient',
  'Proficient',
  'Proficient',
  'Proficient'
);

INSERT INTO character_weapon_proficiencies (
  channel_id,
  user_id,
  weapon_name
) VALUES (
  '644028953719209984',
  '638138236874522634',
  'Battleaxe'
);

INSERT INTO character_weapon_proficiencies (
  channel_id,
  user_id,
  weapon_name
) VALUES (
  '644028953719209984',
  '638138236874522634',
  'Club'
);

INSERT INTO character_weapon_proficiencies (
  channel_id,
  user_id,
  weapon_name
) VALUES (
  '644028953719209984',
  '638138236874522634',
  'Dagger'
);

INSERT INTO character_weapon_proficiencies (
  channel_id,
  user_id,
  weapon_name
) VALUES (
  '644028953719209984',
  '638138236874522634',
  'Dart'
);

INSERT INTO character_weapon_proficiencies (
  channel_id,
  user_id,
  weapon_name
) VALUES (
  '644028953719209984',
  '638138236874522634',
  'Handaxe'
);

INSERT INTO character_weapon_proficiencies (
  channel_id,
  user_id,
  weapon_name
) VALUES (
  '644028953719209984',
  '638138236874522634',
  'Javelin'
);

INSERT INTO character_weapon_proficiencies (
  channel_id,
  user_id,
  weapon_name
) VALUES (
  '644028953719209984',
  '638138236874522634',
  'Light Hammer'
);

INSERT INTO character_weapon_proficiencies (
  channel_id,
  user_id,
  weapon_name
) VALUES (
  '644028953719209984',
  '638138236874522634',
  'Mace'
);

INSERT INTO character_weapon_proficiencies (
  channel_id,
  user_id,
  weapon_name
) VALUES (
  '644028953719209984',
  '638138236874522634',
  'Quarterstaff'
);

INSERT INTO character_weapon_proficiencies (
  channel_id,
  user_id,
  weapon_name
) VALUES (
  '644028953719209984',
  '638138236874522634',
  'Scimitar'
);

INSERT INTO character_weapon_proficiencies (
  channel_id,
  user_id,
  weapon_name
) VALUES (
  '644028953719209984',
  '638138236874522634',
  'Sickle'
);

INSERT INTO character_weapon_proficiencies (
  channel_id,
  user_id,
  weapon_name
) VALUES (
  '644028953719209984',
  '638138236874522634',
  'Sling'
);

INSERT INTO character_weapon_proficiencies (
  channel_id,
  user_id,
  weapon_name
) VALUES (
  '644028953719209984',
  '638138236874522634',
  'Spear'
);

INSERT INTO character_weapon_proficiencies (
  channel_id,
  user_id,
  weapon_name
) VALUES (
  '644028953719209984',
  '638138236874522634',
  'Warhammer'
);

-- Mallile
INSERT INTO characters (
  channel_id,
  user_id,
  level,
  strength,
  dexterity,
  constitution,
  intelligence,
  wisdom,
  charisma,
  intelligence_saving_proficiency,
  wisdom_saving_proficiency,
  arcana_proficiency,
  history_proficiency,
  investigation_proficiency,
  stealth_proficiency
) VALUES (
  '644028953719209984',
  '273957930129424385',
  5,
  10,
  10,
  13,
  16,
  15,
  13,
  true,
  true,
  'Proficient',
  'Proficient',
  'Proficient',
  'Proficient'
);

INSERT INTO character_weapon_proficiencies (
  channel_id,
  user_id,
  weapon_name
) VALUES (
  '644028953719209984',
  '273957930129424385',
  'Light Crossbow'
);

INSERT INTO character_weapon_proficiencies (
  channel_id,
  user_id,
  weapon_name
) VALUES (
  '644028953719209984',
  '273957930129424385',
  'Dagger'
);

INSERT INTO character_weapon_proficiencies (
  channel_id,
  user_id,
  weapon_name
) VALUES (
  '644028953719209984',
  '273957930129424385',
  'Dart'
);

INSERT INTO character_weapon_proficiencies (
  channel_id,
  user_id,
  weapon_name
) VALUES (
  '644028953719209984',
  '273957930129424385',
  'Quarterstaff'
);

INSERT INTO character_weapon_proficiencies (
  channel_id,
  user_id,
  weapon_name
) VALUES (
  '644028953719209984',
  '273957930129424385',
  'Sling'
);
