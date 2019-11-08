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
  survival_proficiency TEXT NOT NULL DEFAULT 'Normal'
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
