CREATE TABLE characters (
  channel_id TEXT NOT NULL,
  user_id TEXT NOT NULL,

  -- Level
  level INTEGER NULL,

  -- Jack of All Trades
  jack_of_all_trades BOOLEAN NOT NULL DEFAULT false,

  -- Abilities
  strength INTEGER NULL,
  dexterity INTEGER NULL,
  constitution INTEGER NULL,
  intelligence INTEGER NULL,
  wisdom INTEGER NULL,
  charisma INTEGER NULL,

  -- Saving Throws
  strength_saving_proficiency BOOLEAN NOT NULL DEFAULT false,
  dexterity_saving_proficiency BOOLEAN NOT NULL DEFAULT false,
  constitution_saving_proficiency BOOLEAN NOT NULL DEFAULT false,
  intelligence_saving_proficiency BOOLEAN NOT NULL DEFAULT false,
  wisdom_saving_proficiency BOOLEAN NOT NULL DEFAULT false,
  charisma_saving_proficiency BOOLEAN NOT NULL DEFAULT false,

  -- Skills
  acrobatics_proficiency TEXT NOT NULL DEFAULT 'Normal',
  animal_handling_proficiency TEXT NOT NULL DEFAULT 'Normal',
  arcana_proficiency TEXT NOT NULL DEFAULT 'Normal',
  athletics_proficiency TEXT NOT NULL DEFAULT 'Normal',
  deception_proficiency TEXT NOT NULL DEFAULT 'Normal',
  history_proficiency TEXT NOT NULL DEFAULT 'Normal',
  insight_proficiency TEXT NOT NULL DEFAULT 'Normal',
  intimidation_proficiency TEXT NOT NULL DEFAULT 'Normal',
  investigation_proficiency TEXT NOT NULL DEFAULT 'Normal',
  medicine_proficiency TEXT NOT NULL DEFAULT 'Normal',
  nature_proficiency TEXT NOT NULL DEFAULT 'Normal',
  perception_proficiency TEXT NOT NULL DEFAULT 'Normal',
  performance_proficiency TEXT NOT NULL DEFAULT 'Normal',
  persuasion_proficiency TEXT NOT NULL DEFAULT 'Normal',
  religion_proficiency TEXT NOT NULL DEFAULT 'Normal',
  sleight_of_hand_proficiency TEXT NOT NULL DEFAULT 'Normal',
  stealth_proficiency TEXT NOT NULL DEFAULT 'Normal',
  survival_proficiency TEXT NOT NULL DEFAULT 'Normal',

  PRIMARY KEY (channel_id, user_id)
);

INSERT INTO characters (
  channel_id,
  user_id,
  level,
  strength,
  dexterity,
  constitution,
  intelligence,
  wisdom,
  charisma
) VALUES (
  '641788813647151113',
  '292225680303849472',
  1,
  18,
  13,
  16,
  8,
  10,
  12
);

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
