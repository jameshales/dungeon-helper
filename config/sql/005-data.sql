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

-- Softly Chiming Bells
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
  arcana_proficiency,
  history_proficiency,
  insight_proficiency,
  investigation_proficiency,
  sleight_of_hand_proficiency,
  stealth_proficiency
) VALUES (
  '644028953719209984',
  '194800877960167424',
  1,
  8,
  17,
  14,
  10,
  14,
  12,
  true,
  true,
  'Proficient',
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

-- Loe'Guo
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
  strength_saving_proficiency,
  dexterity_saving_proficiency,
  athletics_proficiency,
  history_proficiency,
  medicine_proficiency,
  religion_proficiency,
  survival_proficiency
) VALUES (
  '644028953719209984',
  '583587280296345601',
  1,
  false,
  16,
  8,
  13,
  12,
  16,
  10,
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
  'Shortsword'
);

-- To-cha
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
  dexterity_saving_proficiency,
  insight_proficiency,
  nature_proficiency,
  perception_proficiency,
  stealth_proficiency,
  survival_proficiency
) VALUES (
  '644028953719209984',
  '638138236874522634',
  1,
  14,
  16,
  12,
  12,
  14,
  8,
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
  '638138236874522634',
  'Simple'
);

INSERT INTO character_weapon_proficiencies (
  channel_id,
  user_id,
  weapon_category
) VALUES (
  '644028953719209984',
  '638138236874522634',
  'Martial'
);

-- Connie Yellostone
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
  deception_proficiency,
  insight_proficiency,
  investigation_proficiency,
  nature_proficiency,
  persuasion_proficiency,
  stealth_proficiency
) VALUES (
  '644028953719209984',
  '273957930129424385',
  1,
  15,
  8,
  14,
  10,
  16,
  12,
  true,
  true,
  'Proficient',
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
  '273957930129424385',
  'Simple'
);

-- Achencheres
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
  dexterity_saving_proficiency,
  athletics_proficiency,
  insight_proficiency,
  perception_proficiency,
  religion_proficiency,
  stealth_proficiency,
  survival_proficiency
) VALUES (
  '644028953719209984',
  '266082699599544320',
  1,
  12,
  16,
  14,
  8,
  16,
  8,
  true,
  true,
  'Proficient',
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
  '266082699599544320',
  'Simple'
);

INSERT INTO character_weapon_proficiencies (
  channel_id,
  user_id,
  weapon_name
) VALUES (
  '644028953719209984',
  '266082699599544320',
  'Shortsword'
);
