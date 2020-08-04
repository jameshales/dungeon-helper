use crate::weapon::{Category, WeaponName};
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use rusqlite::Result as RusqliteResult;
use rusqlite::{Connection, OptionalExtension, Row};
use serenity::model::id::{ChannelId, UserId};
use std::error;
use std::fmt;

/// A character in a Dungeons and Dragons campaign.
///
/// The character has a number of base abilities and proficiencies, from which ability and
/// skill modifiers are calculated.
#[derive(Debug, Eq, PartialEq)]
pub struct Character {
    level: Option<i32>,
    jack_of_all_trades: bool,
    martial_arts: bool,

    // Abilities
    strength: Option<i32>,
    dexterity: Option<i32>,
    constitution: Option<i32>,
    intelligence: Option<i32>,
    wisdom: Option<i32>,
    charisma: Option<i32>,

    // Saving Throws
    strength_saving_proficiency: bool,
    dexterity_saving_proficiency: bool,
    constitution_saving_proficiency: bool,
    intelligence_saving_proficiency: bool,
    wisdom_saving_proficiency: bool,
    charisma_saving_proficiency: bool,

    // Skills
    acrobatics_proficiency: Proficiency,
    animal_handling_proficiency: Proficiency,
    arcana_proficiency: Proficiency,
    athletics_proficiency: Proficiency,
    deception_proficiency: Proficiency,
    history_proficiency: Proficiency,
    insight_proficiency: Proficiency,
    intimidation_proficiency: Proficiency,
    investigation_proficiency: Proficiency,
    medicine_proficiency: Proficiency,
    nature_proficiency: Proficiency,
    perception_proficiency: Proficiency,
    performance_proficiency: Proficiency,
    persuasion_proficiency: Proficiency,
    religion_proficiency: Proficiency,
    sleight_of_hand_proficiency: Proficiency,
    stealth_proficiency: Proficiency,
    survival_proficiency: Proficiency,
}

impl Character {
    pub fn get(
        connection: &Connection,
        channel_id: ChannelId,
        user_id: UserId,
    ) -> RusqliteResult<Option<Character>> {
        connection
            .query_row(
                "SELECT \
                 level, \
                 jack_of_all_trades, \
                 martial_arts, \
                 strength, \
                 dexterity, \
                 constitution, \
                 intelligence, \
                 wisdom, \
                 charisma, \
                 strength_saving_proficiency, \
                 dexterity_saving_proficiency, \
                 constitution_saving_proficiency, \
                 intelligence_saving_proficiency, \
                 wisdom_saving_proficiency, \
                 charisma_saving_proficiency, \
                 acrobatics_proficiency, \
                 animal_handling_proficiency, \
                 arcana_proficiency, \
                 athletics_proficiency, \
                 deception_proficiency, \
                 history_proficiency, \
                 insight_proficiency, \
                 intimidation_proficiency, \
                 investigation_proficiency, \
                 medicine_proficiency, \
                 nature_proficiency, \
                 perception_proficiency, \
                 performance_proficiency, \
                 persuasion_proficiency, \
                 religion_proficiency, \
                 sleight_of_hand_proficiency, \
                 stealth_proficiency, \
                 survival_proficiency \
                 FROM characters \
                 WHERE channel_id = $1 \
                 AND user_id = $2",
                &[&channel_id.to_string(), &user_id.to_string()],
                Character::from_row,
            )
            .optional()
    }

    pub fn from_row(row: &Row) -> RusqliteResult<Character> {
        Ok(Character {
            level: row.get("level")?,
            jack_of_all_trades: row.get("jack_of_all_trades")?,
            martial_arts: row.get("martial_arts")?,

            strength: row.get("strength")?,
            dexterity: row.get("dexterity")?,
            constitution: row.get("constitution")?,
            intelligence: row.get("intelligence")?,
            wisdom: row.get("wisdom")?,
            charisma: row.get("charisma")?,

            strength_saving_proficiency: row.get("strength_saving_proficiency")?,
            dexterity_saving_proficiency: row.get("dexterity_saving_proficiency")?,
            constitution_saving_proficiency: row.get("constitution_saving_proficiency")?,
            intelligence_saving_proficiency: row.get("intelligence_saving_proficiency")?,
            wisdom_saving_proficiency: row.get("wisdom_saving_proficiency")?,
            charisma_saving_proficiency: row.get("charisma_saving_proficiency")?,

            acrobatics_proficiency: row.get("acrobatics_proficiency")?,
            animal_handling_proficiency: row.get("animal_handling_proficiency")?,
            arcana_proficiency: row.get("arcana_proficiency")?,
            athletics_proficiency: row.get("athletics_proficiency")?,
            deception_proficiency: row.get("deception_proficiency")?,
            history_proficiency: row.get("history_proficiency")?,
            insight_proficiency: row.get("insight_proficiency")?,
            intimidation_proficiency: row.get("intimidation_proficiency")?,
            investigation_proficiency: row.get("investigation_proficiency")?,
            medicine_proficiency: row.get("medicine_proficiency")?,
            nature_proficiency: row.get("nature_proficiency")?,
            perception_proficiency: row.get("perception_proficiency")?,
            performance_proficiency: row.get("performance_proficiency")?,
            persuasion_proficiency: row.get("persuasion_proficiency")?,
            religion_proficiency: row.get("religion_proficiency")?,
            sleight_of_hand_proficiency: row.get("sleight_of_hand_proficiency")?,
            stealth_proficiency: row.get("stealth_proficiency")?,
            survival_proficiency: row.get("survival_proficiency")?,
        })
    }

    pub fn martial_arts(&self) -> bool {
        self.martial_arts
    }

    pub fn martial_arts_damage_die(&self) -> Option<i32> {
        if self.martial_arts {
            Some(2 * ((self.level? + 1) / 6) + 4)
        } else {
            None
        }
    }

    pub fn proficiency_bonus(&self) -> Option<i32> {
        self.level.map(|level| (level - 1) / 4 + 2)
    }

    // Abilities

    pub fn ability(&self, name: AbilityName) -> Option<Ability> {
        match name {
            AbilityName::Strength => self.strength(),
            AbilityName::Dexterity => self.dexterity(),
            AbilityName::Constitution => self.constitution(),
            AbilityName::Intelligence => self.intelligence(),
            AbilityName::Wisdom => self.wisdom(),
            AbilityName::Charisma => self.charisma(),
        }
    }

    pub fn strength(&self) -> Option<Ability> {
        Character::make_ability(self.strength)
    }

    pub fn dexterity(&self) -> Option<Ability> {
        Character::make_ability(self.dexterity)
    }

    pub fn constitution(&self) -> Option<Ability> {
        Character::make_ability(self.constitution)
    }

    pub fn intelligence(&self) -> Option<Ability> {
        Character::make_ability(self.intelligence)
    }

    pub fn wisdom(&self) -> Option<Ability> {
        Character::make_ability(self.wisdom)
    }

    pub fn charisma(&self) -> Option<Ability> {
        Character::make_ability(self.charisma)
    }

    fn make_ability(score: Option<i32>) -> Option<Ability> {
        Some(Ability {
            score: score?,
            modifier: score? / 2 - 5,
        })
    }

    // Saving Throws

    pub fn saving_throw(&self, name: AbilityName) -> Option<SavingThrow> {
        match name {
            AbilityName::Strength => self.strength_saving_throw(),
            AbilityName::Dexterity => self.dexterity_saving_throw(),
            AbilityName::Constitution => self.constitution_saving_throw(),
            AbilityName::Intelligence => self.intelligence_saving_throw(),
            AbilityName::Wisdom => self.wisdom_saving_throw(),
            AbilityName::Charisma => self.charisma_saving_throw(),
        }
    }

    pub fn strength_saving_throw(&self) -> Option<SavingThrow> {
        self.make_saving_throw(self.strength(), self.strength_saving_proficiency)
    }

    pub fn dexterity_saving_throw(&self) -> Option<SavingThrow> {
        self.make_saving_throw(self.dexterity(), self.dexterity_saving_proficiency)
    }

    pub fn constitution_saving_throw(&self) -> Option<SavingThrow> {
        self.make_saving_throw(self.constitution(), self.constitution_saving_proficiency)
    }

    pub fn intelligence_saving_throw(&self) -> Option<SavingThrow> {
        self.make_saving_throw(self.intelligence(), self.intelligence_saving_proficiency)
    }

    pub fn wisdom_saving_throw(&self) -> Option<SavingThrow> {
        self.make_saving_throw(self.wisdom(), self.wisdom_saving_proficiency)
    }

    pub fn charisma_saving_throw(&self) -> Option<SavingThrow> {
        self.make_saving_throw(self.charisma(), self.charisma_saving_proficiency)
    }

    fn make_saving_throw(
        &self,
        ability: Option<Ability>,
        proficiency: bool,
    ) -> Option<SavingThrow> {
        let bonus = if proficiency {
            self.proficiency_bonus()?
        } else {
            0
        };
        Some(SavingThrow {
            modifier: ability?.modifier + bonus,
            proficiency,
        })
    }

    // Skills

    pub fn skill(&self, name: SkillName) -> Option<Skill> {
        match name {
            SkillName::Acrobatics => self.acrobatics(),
            SkillName::AnimalHandling => self.animal_handling(),
            SkillName::Arcana => self.arcana(),
            SkillName::Athletics => self.athletics(),
            SkillName::Deception => self.deception(),
            SkillName::History => self.history(),
            SkillName::Insight => self.insight(),
            SkillName::Intimidation => self.intimidation(),
            SkillName::Investigation => self.investigation(),
            SkillName::Medicine => self.medicine(),
            SkillName::Nature => self.nature(),
            SkillName::Perception => self.perception(),
            SkillName::Performance => self.performance(),
            SkillName::Persuasion => self.persuasion(),
            SkillName::Religion => self.religion(),
            SkillName::SleightOfHand => self.sleight_of_hand(),
            SkillName::Stealth => self.stealth(),
            SkillName::Survival => self.survival(),
        }
    }

    pub fn acrobatics(&self) -> Option<Skill> {
        self.make_skill(self.dexterity(), self.acrobatics_proficiency)
    }

    pub fn animal_handling(&self) -> Option<Skill> {
        self.make_skill(self.wisdom(), self.animal_handling_proficiency)
    }

    pub fn arcana(&self) -> Option<Skill> {
        self.make_skill(self.intelligence(), self.arcana_proficiency)
    }

    pub fn athletics(&self) -> Option<Skill> {
        self.make_skill(self.strength(), self.athletics_proficiency)
    }

    pub fn deception(&self) -> Option<Skill> {
        self.make_skill(self.charisma(), self.deception_proficiency)
    }

    pub fn history(&self) -> Option<Skill> {
        self.make_skill(self.intelligence(), self.history_proficiency)
    }

    pub fn insight(&self) -> Option<Skill> {
        self.make_skill(self.wisdom(), self.insight_proficiency)
    }

    pub fn intimidation(&self) -> Option<Skill> {
        self.make_skill(self.charisma(), self.intimidation_proficiency)
    }

    pub fn investigation(&self) -> Option<Skill> {
        self.make_skill(self.intelligence(), self.investigation_proficiency)
    }

    pub fn medicine(&self) -> Option<Skill> {
        self.make_skill(self.wisdom(), self.medicine_proficiency)
    }

    pub fn nature(&self) -> Option<Skill> {
        self.make_skill(self.intelligence(), self.nature_proficiency)
    }

    pub fn perception(&self) -> Option<Skill> {
        self.make_skill(self.wisdom(), self.perception_proficiency)
    }

    pub fn performance(&self) -> Option<Skill> {
        self.make_skill(self.charisma(), self.performance_proficiency)
    }

    pub fn persuasion(&self) -> Option<Skill> {
        self.make_skill(self.charisma(), self.persuasion_proficiency)
    }

    pub fn religion(&self) -> Option<Skill> {
        self.make_skill(self.intelligence(), self.religion_proficiency)
    }

    pub fn sleight_of_hand(&self) -> Option<Skill> {
        self.make_skill(self.dexterity(), self.sleight_of_hand_proficiency)
    }

    pub fn stealth(&self) -> Option<Skill> {
        self.make_skill(self.dexterity(), self.stealth_proficiency)
    }

    pub fn survival(&self) -> Option<Skill> {
        self.make_skill(self.wisdom(), self.survival_proficiency)
    }

    fn make_skill(&self, ability: Option<Ability>, proficiency: Proficiency) -> Option<Skill> {
        let proficiency_bonus = self.proficiency_bonus()?;
        let bonus = match proficiency {
            Proficiency::Normal if !self.jack_of_all_trades => 0,
            Proficiency::Normal => proficiency_bonus / 2,
            Proficiency::Proficient => proficiency_bonus,
            Proficiency::Expert => 2 * proficiency_bonus,
        };
        Some(Skill {
            modifier: ability?.modifier + bonus,
            proficiency,
        })
    }

    pub fn has_weapon_proficiency(
        connection: &Connection,
        channel_id: ChannelId,
        user_id: UserId,
        name: WeaponName,
        category: Category,
    ) -> RusqliteResult<bool> {
        let params: &[&dyn ToSql] = &[
            &channel_id.to_string(),
            &user_id.to_string(),
            &name.as_str(),
            &category.as_str(),
        ];
        connection
            .query_row(
                "SELECT true \
             FROM character_weapon_proficiencies \
             WHERE channel_id = $1 \
             AND user_id = $2
             AND (weapon_name = $3 OR weapon_category = $4)",
                params,
                |row| row.get(0),
            )
            .optional()
            .map(|result| result.unwrap_or(false))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Proficiency {
    Normal,
    Proficient,
    Expert,
}

impl Proficiency {
    pub fn parse(string: &str) -> Option<Proficiency> {
        match string.to_lowercase().as_ref() {
            "normal" => Some(Proficiency::Normal),
            "proficient" => Some(Proficiency::Proficient),
            "expert" => Some(Proficiency::Expert),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Proficiency::Normal => "Normal",
            Proficiency::Proficient => "Proficient",
            Proficiency::Expert => "Expert",
        }
    }
}

impl FromSql for Proficiency {
    fn column_result(value: ValueRef) -> FromSqlResult<Proficiency> {
        value.as_str().and_then(|string| {
            Proficiency::parse(string).ok_or_else(|| {
                FromSqlError::Other(Box::new(InvalidProficiencyValueError {
                    value: string.to_owned(),
                }))
            })
        })
    }
}

impl ToSql for Proficiency {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput> {
        self.as_str().to_sql()
    }
}

#[derive(Debug)]
struct InvalidProficiencyValueError {
    value: String,
}

impl fmt::Display for InvalidProficiencyValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid value for proficiency (value = {})", self.value)
    }
}

impl error::Error for InvalidProficiencyValueError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Ability {
    pub score: i32,
    pub modifier: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SavingThrow {
    pub modifier: i32,
    pub proficiency: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Skill {
    pub modifier: i32,
    pub proficiency: Proficiency,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AbilityName {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

impl AbilityName {
    pub fn parse(string: &str) -> Option<AbilityName> {
        match string.to_lowercase().as_ref() {
            "str" | "strength" => Some(AbilityName::Strength),
            "dex" | "dexterity" => Some(AbilityName::Dexterity),
            "con" | "constitution" => Some(AbilityName::Constitution),
            "int" | "intelligence" => Some(AbilityName::Intelligence),
            "wis" | "wisdom" => Some(AbilityName::Wisdom),
            "cha" | "charisma" => Some(AbilityName::Charisma),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            AbilityName::Strength => "Strength",
            AbilityName::Dexterity => "Dexterity",
            AbilityName::Constitution => "Constitution",
            AbilityName::Intelligence => "Intelligence",
            AbilityName::Wisdom => "Wisdom",
            AbilityName::Charisma => "Charisma",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SkillName {
    Acrobatics,
    AnimalHandling,
    Arcana,
    Athletics,
    Deception,
    History,
    Insight,
    Intimidation,
    Investigation,
    Medicine,
    Nature,
    Perception,
    Performance,
    Persuasion,
    Religion,
    SleightOfHand,
    Stealth,
    Survival,
}

impl SkillName {
    pub fn parse(string: &str) -> Option<SkillName> {
        match string.to_lowercase().as_ref() {
            "acrobatics" => Some(SkillName::Acrobatics),
            "animal handling" => Some(SkillName::AnimalHandling),
            "arcana" => Some(SkillName::Arcana),
            "athletics" => Some(SkillName::Athletics),
            "deception" => Some(SkillName::Deception),
            "history" => Some(SkillName::History),
            "insight" => Some(SkillName::Insight),
            "intimidation" => Some(SkillName::Intimidation),
            "investigation" => Some(SkillName::Investigation),
            "medicine" => Some(SkillName::Medicine),
            "nature" => Some(SkillName::Nature),
            "perception" => Some(SkillName::Perception),
            "performance" => Some(SkillName::Performance),
            "persuasion" => Some(SkillName::Persuasion),
            "religion" => Some(SkillName::Religion),
            "sleight of hand" => Some(SkillName::SleightOfHand),
            "stealth" => Some(SkillName::Stealth),
            "survival" => Some(SkillName::Survival),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            SkillName::Acrobatics => "Acrobatics",
            SkillName::AnimalHandling => "Animal Handling",
            SkillName::Arcana => "Arcana",
            SkillName::Athletics => "Athletics",
            SkillName::Deception => "Deception",
            SkillName::History => "History",
            SkillName::Insight => "Insight",
            SkillName::Intimidation => "Intimidation",
            SkillName::Investigation => "Investigation",
            SkillName::Medicine => "Medicine",
            SkillName::Nature => "Nature",
            SkillName::Perception => "Perception",
            SkillName::Performance => "Performance",
            SkillName::Persuasion => "Persuasion",
            SkillName::Religion => "Religion",
            SkillName::SleightOfHand => "Sleight Of Hand",
            SkillName::Stealth => "Stealth",
            SkillName::Survival => "Survival",
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_profiency_bonus() {
        fn character(level: Option<i32>) -> Character {
            Character {
                level,
                jack_of_all_trades: false,
                martial_arts: false,

                strength: None,
                dexterity: None,
                constitution: None,
                intelligence: None,
                wisdom: None,
                charisma: None,

                strength_saving_proficiency: false,
                dexterity_saving_proficiency: false,
                constitution_saving_proficiency: false,
                intelligence_saving_proficiency: false,
                wisdom_saving_proficiency: false,
                charisma_saving_proficiency: false,

                acrobatics_proficiency: Proficiency::Normal,
                animal_handling_proficiency: Proficiency::Normal,
                arcana_proficiency: Proficiency::Normal,
                athletics_proficiency: Proficiency::Normal,
                deception_proficiency: Proficiency::Normal,
                history_proficiency: Proficiency::Normal,
                insight_proficiency: Proficiency::Normal,
                intimidation_proficiency: Proficiency::Normal,
                investigation_proficiency: Proficiency::Normal,
                medicine_proficiency: Proficiency::Normal,
                nature_proficiency: Proficiency::Normal,
                perception_proficiency: Proficiency::Normal,
                performance_proficiency: Proficiency::Normal,
                persuasion_proficiency: Proficiency::Normal,
                religion_proficiency: Proficiency::Normal,
                sleight_of_hand_proficiency: Proficiency::Normal,
                stealth_proficiency: Proficiency::Normal,
                survival_proficiency: Proficiency::Normal,
            }
        }

        assert_eq!(character(None).proficiency_bonus(), None);
        assert_eq!(character(Some(1)).proficiency_bonus(), Some(2));
        assert_eq!(character(Some(4)).proficiency_bonus(), Some(2));
        assert_eq!(character(Some(5)).proficiency_bonus(), Some(3));
        assert_eq!(character(Some(8)).proficiency_bonus(), Some(3));
        assert_eq!(character(Some(9)).proficiency_bonus(), Some(4));
    }

    #[test]
    fn test_martial_arts_damage_die() {
        fn character(level: Option<i32>, martial_arts: bool) -> Character {
            Character {
                level,
                jack_of_all_trades: false,
                martial_arts,

                strength: None,
                dexterity: None,
                constitution: None,
                intelligence: None,
                wisdom: None,
                charisma: None,

                strength_saving_proficiency: false,
                dexterity_saving_proficiency: false,
                constitution_saving_proficiency: false,
                intelligence_saving_proficiency: false,
                wisdom_saving_proficiency: false,
                charisma_saving_proficiency: false,

                acrobatics_proficiency: Proficiency::Normal,
                animal_handling_proficiency: Proficiency::Normal,
                arcana_proficiency: Proficiency::Normal,
                athletics_proficiency: Proficiency::Normal,
                deception_proficiency: Proficiency::Normal,
                history_proficiency: Proficiency::Normal,
                insight_proficiency: Proficiency::Normal,
                intimidation_proficiency: Proficiency::Normal,
                investigation_proficiency: Proficiency::Normal,
                medicine_proficiency: Proficiency::Normal,
                nature_proficiency: Proficiency::Normal,
                perception_proficiency: Proficiency::Normal,
                performance_proficiency: Proficiency::Normal,
                persuasion_proficiency: Proficiency::Normal,
                religion_proficiency: Proficiency::Normal,
                sleight_of_hand_proficiency: Proficiency::Normal,
                stealth_proficiency: Proficiency::Normal,
                survival_proficiency: Proficiency::Normal,
            }
        }

        assert_eq!(character(None, false).martial_arts_damage_die(), None);
        assert_eq!(character(Some(1), false).martial_arts_damage_die(), None);
        assert_eq!(character(None, true).martial_arts_damage_die(), None);
        assert_eq!(character(Some(1), true).martial_arts_damage_die(), Some(4));
        assert_eq!(character(Some(4), true).martial_arts_damage_die(), Some(4));
        assert_eq!(character(Some(5), true).martial_arts_damage_die(), Some(6));
        assert_eq!(character(Some(10), true).martial_arts_damage_die(), Some(6));
        assert_eq!(character(Some(11), true).martial_arts_damage_die(), Some(8));
        assert_eq!(character(Some(16), true).martial_arts_damage_die(), Some(8));
        assert_eq!(
            character(Some(17), true).martial_arts_damage_die(),
            Some(10)
        );
        assert_eq!(
            character(Some(20), true).martial_arts_damage_die(),
            Some(10)
        );
    }

    #[test]
    fn test_strength() {
        fn character(strength: Option<i32>) -> Character {
            Character {
                level: None,
                jack_of_all_trades: false,
                martial_arts: false,

                strength,
                dexterity: None,
                constitution: None,
                intelligence: None,
                wisdom: None,
                charisma: None,

                strength_saving_proficiency: false,
                dexterity_saving_proficiency: false,
                constitution_saving_proficiency: false,
                intelligence_saving_proficiency: false,
                wisdom_saving_proficiency: false,
                charisma_saving_proficiency: false,

                acrobatics_proficiency: Proficiency::Normal,
                animal_handling_proficiency: Proficiency::Normal,
                arcana_proficiency: Proficiency::Normal,
                athletics_proficiency: Proficiency::Normal,
                deception_proficiency: Proficiency::Normal,
                history_proficiency: Proficiency::Normal,
                insight_proficiency: Proficiency::Normal,
                intimidation_proficiency: Proficiency::Normal,
                investigation_proficiency: Proficiency::Normal,
                medicine_proficiency: Proficiency::Normal,
                nature_proficiency: Proficiency::Normal,
                perception_proficiency: Proficiency::Normal,
                performance_proficiency: Proficiency::Normal,
                persuasion_proficiency: Proficiency::Normal,
                religion_proficiency: Proficiency::Normal,
                sleight_of_hand_proficiency: Proficiency::Normal,
                stealth_proficiency: Proficiency::Normal,
                survival_proficiency: Proficiency::Normal,
            }
        }

        assert_eq!(character(None).strength(), None);
        assert_eq!(
            character(Some(1)).strength(),
            Some(Ability {
                score: 1,
                modifier: -5
            })
        );
        assert_eq!(
            character(Some(2)).strength(),
            Some(Ability {
                score: 2,
                modifier: -4
            })
        );
        assert_eq!(
            character(Some(3)).strength(),
            Some(Ability {
                score: 3,
                modifier: -4
            })
        );
        assert_eq!(
            character(Some(8)).strength(),
            Some(Ability {
                score: 8,
                modifier: -1
            })
        );
        assert_eq!(
            character(Some(9)).strength(),
            Some(Ability {
                score: 9,
                modifier: -1
            })
        );
        assert_eq!(
            character(Some(10)).strength(),
            Some(Ability {
                score: 10,
                modifier: 0
            })
        );
        assert_eq!(
            character(Some(11)).strength(),
            Some(Ability {
                score: 11,
                modifier: 0
            })
        );
        assert_eq!(
            character(Some(12)).strength(),
            Some(Ability {
                score: 12,
                modifier: 1
            })
        );
        assert_eq!(
            character(Some(13)).strength(),
            Some(Ability {
                score: 13,
                modifier: 1
            })
        );
        assert_eq!(
            character(Some(29)).strength(),
            Some(Ability {
                score: 29,
                modifier: 9
            })
        );
        assert_eq!(
            character(Some(30)).strength(),
            Some(Ability {
                score: 30,
                modifier: 10
            })
        );
    }

    #[test]
    fn test_saving_throw() {
        fn character(strength: Option<i32>) -> Character {
            Character {
                level: None,
                jack_of_all_trades: false,
                martial_arts: false,

                strength,
                dexterity: None,
                constitution: None,
                intelligence: None,
                wisdom: None,
                charisma: None,

                strength_saving_proficiency: false,
                dexterity_saving_proficiency: false,
                constitution_saving_proficiency: false,
                intelligence_saving_proficiency: false,
                wisdom_saving_proficiency: false,
                charisma_saving_proficiency: false,

                acrobatics_proficiency: Proficiency::Normal,
                animal_handling_proficiency: Proficiency::Normal,
                arcana_proficiency: Proficiency::Normal,
                athletics_proficiency: Proficiency::Normal,
                deception_proficiency: Proficiency::Normal,
                history_proficiency: Proficiency::Normal,
                insight_proficiency: Proficiency::Normal,
                intimidation_proficiency: Proficiency::Normal,
                investigation_proficiency: Proficiency::Normal,
                medicine_proficiency: Proficiency::Normal,
                nature_proficiency: Proficiency::Normal,
                perception_proficiency: Proficiency::Normal,
                performance_proficiency: Proficiency::Normal,
                persuasion_proficiency: Proficiency::Normal,
                religion_proficiency: Proficiency::Normal,
                sleight_of_hand_proficiency: Proficiency::Normal,
                stealth_proficiency: Proficiency::Normal,
                survival_proficiency: Proficiency::Normal,
            }
        }

        assert_eq!(character(None).saving_throw(AbilityName::Strength), None);
        assert_eq!(
            character(Some(1)).saving_throw(AbilityName::Strength),
            Some(SavingThrow {
                modifier: -5,
                proficiency: false
            })
        );
        assert_eq!(
            character(Some(2)).saving_throw(AbilityName::Strength),
            Some(SavingThrow {
                modifier: -4,
                proficiency: false
            })
        );
        assert_eq!(
            character(Some(3)).saving_throw(AbilityName::Strength),
            Some(SavingThrow {
                modifier: -4,
                proficiency: false
            })
        );
        assert_eq!(
            character(Some(8)).saving_throw(AbilityName::Strength),
            Some(SavingThrow {
                modifier: -1,
                proficiency: false
            })
        );
        assert_eq!(
            character(Some(9)).saving_throw(AbilityName::Strength),
            Some(SavingThrow {
                modifier: -1,
                proficiency: false
            })
        );
        assert_eq!(
            character(Some(10)).saving_throw(AbilityName::Strength),
            Some(SavingThrow {
                modifier: 0,
                proficiency: false
            })
        );
        assert_eq!(
            character(Some(11)).saving_throw(AbilityName::Strength),
            Some(SavingThrow {
                modifier: 0,
                proficiency: false
            })
        );
        assert_eq!(
            character(Some(12)).saving_throw(AbilityName::Strength),
            Some(SavingThrow {
                modifier: 1,
                proficiency: false
            })
        );
        assert_eq!(
            character(Some(13)).saving_throw(AbilityName::Strength),
            Some(SavingThrow {
                modifier: 1,
                proficiency: false
            })
        );
        assert_eq!(
            character(Some(29)).saving_throw(AbilityName::Strength),
            Some(SavingThrow {
                modifier: 9,
                proficiency: false
            })
        );
        assert_eq!(
            character(Some(30)).saving_throw(AbilityName::Strength),
            Some(SavingThrow {
                modifier: 10,
                proficiency: false
            })
        );
    }

    #[test]
    fn test_skill() {
        fn character(
            level: Option<i32>,
            strength: Option<i32>,
            athletics_proficiency: Proficiency,
        ) -> Character {
            Character {
                level,
                jack_of_all_trades: false,
                martial_arts: false,

                strength,
                dexterity: None,
                constitution: None,
                intelligence: None,
                wisdom: None,
                charisma: None,

                strength_saving_proficiency: false,
                dexterity_saving_proficiency: false,
                constitution_saving_proficiency: false,
                intelligence_saving_proficiency: false,
                wisdom_saving_proficiency: false,
                charisma_saving_proficiency: false,

                acrobatics_proficiency: Proficiency::Normal,
                animal_handling_proficiency: Proficiency::Normal,
                arcana_proficiency: Proficiency::Normal,
                athletics_proficiency,
                deception_proficiency: Proficiency::Normal,
                history_proficiency: Proficiency::Normal,
                insight_proficiency: Proficiency::Normal,
                intimidation_proficiency: Proficiency::Normal,
                investigation_proficiency: Proficiency::Normal,
                medicine_proficiency: Proficiency::Normal,
                nature_proficiency: Proficiency::Normal,
                perception_proficiency: Proficiency::Normal,
                performance_proficiency: Proficiency::Normal,
                persuasion_proficiency: Proficiency::Normal,
                religion_proficiency: Proficiency::Normal,
                sleight_of_hand_proficiency: Proficiency::Normal,
                stealth_proficiency: Proficiency::Normal,
                survival_proficiency: Proficiency::Normal,
            }
        }

        assert_eq!(
            character(Some(1), Some(1), Proficiency::Normal).skill(SkillName::Athletics),
            Some(Skill {
                modifier: -5,
                proficiency: Proficiency::Normal
            })
        );
        assert_eq!(
            character(Some(1), Some(2), Proficiency::Normal).skill(SkillName::Athletics),
            Some(Skill {
                modifier: -4,
                proficiency: Proficiency::Normal
            })
        );
        assert_eq!(
            character(Some(1), Some(3), Proficiency::Normal).skill(SkillName::Athletics),
            Some(Skill {
                modifier: -4,
                proficiency: Proficiency::Normal
            })
        );
        assert_eq!(
            character(Some(1), Some(4), Proficiency::Normal).skill(SkillName::Athletics),
            Some(Skill {
                modifier: -3,
                proficiency: Proficiency::Normal
            })
        );
        assert_eq!(
            character(Some(1), Some(5), Proficiency::Normal).skill(SkillName::Athletics),
            Some(Skill {
                modifier: -3,
                proficiency: Proficiency::Normal
            })
        );
        assert_eq!(
            character(Some(1), Some(6), Proficiency::Normal).skill(SkillName::Athletics),
            Some(Skill {
                modifier: -2,
                proficiency: Proficiency::Normal
            })
        );
        assert_eq!(
            character(Some(1), Some(7), Proficiency::Normal).skill(SkillName::Athletics),
            Some(Skill {
                modifier: -2,
                proficiency: Proficiency::Normal
            })
        );
        assert_eq!(
            character(Some(1), Some(8), Proficiency::Normal).skill(SkillName::Athletics),
            Some(Skill {
                modifier: -1,
                proficiency: Proficiency::Normal
            })
        );
        assert_eq!(
            character(Some(1), Some(9), Proficiency::Normal).skill(SkillName::Athletics),
            Some(Skill {
                modifier: -1,
                proficiency: Proficiency::Normal
            })
        );
        assert_eq!(
            character(Some(1), Some(10), Proficiency::Normal).skill(SkillName::Athletics),
            Some(Skill {
                modifier: 0,
                proficiency: Proficiency::Normal
            })
        );
        assert_eq!(
            character(Some(1), Some(11), Proficiency::Normal).skill(SkillName::Athletics),
            Some(Skill {
                modifier: 0,
                proficiency: Proficiency::Normal
            })
        );
        assert_eq!(
            character(Some(1), Some(12), Proficiency::Normal).skill(SkillName::Athletics),
            Some(Skill {
                modifier: 1,
                proficiency: Proficiency::Normal
            })
        );
        assert_eq!(
            character(Some(1), Some(13), Proficiency::Normal).skill(SkillName::Athletics),
            Some(Skill {
                modifier: 1,
                proficiency: Proficiency::Normal
            })
        );
        assert_eq!(
            character(Some(1), Some(29), Proficiency::Normal).skill(SkillName::Athletics),
            Some(Skill {
                modifier: 9,
                proficiency: Proficiency::Normal
            })
        );
        assert_eq!(
            character(Some(1), Some(30), Proficiency::Normal).skill(SkillName::Athletics),
            Some(Skill {
                modifier: 10,
                proficiency: Proficiency::Normal
            })
        );
        assert_eq!(
            character(Some(1), Some(1), Proficiency::Proficient).skill(SkillName::Athletics),
            Some(Skill {
                modifier: -3,
                proficiency: Proficiency::Proficient
            })
        );
        assert_eq!(
            character(Some(1), Some(2), Proficiency::Proficient).skill(SkillName::Athletics),
            Some(Skill {
                modifier: -2,
                proficiency: Proficiency::Proficient
            })
        );
        assert_eq!(
            character(Some(1), Some(3), Proficiency::Proficient).skill(SkillName::Athletics),
            Some(Skill {
                modifier: -2,
                proficiency: Proficiency::Proficient
            })
        );
        assert_eq!(
            character(Some(1), Some(4), Proficiency::Proficient).skill(SkillName::Athletics),
            Some(Skill {
                modifier: -1,
                proficiency: Proficiency::Proficient
            })
        );
        assert_eq!(
            character(Some(1), Some(5), Proficiency::Proficient).skill(SkillName::Athletics),
            Some(Skill {
                modifier: -1,
                proficiency: Proficiency::Proficient
            })
        );
        assert_eq!(
            character(Some(1), Some(6), Proficiency::Proficient).skill(SkillName::Athletics),
            Some(Skill {
                modifier: 0,
                proficiency: Proficiency::Proficient
            })
        );
        assert_eq!(
            character(Some(1), Some(7), Proficiency::Proficient).skill(SkillName::Athletics),
            Some(Skill {
                modifier: 0,
                proficiency: Proficiency::Proficient
            })
        );
        assert_eq!(
            character(Some(1), Some(8), Proficiency::Proficient).skill(SkillName::Athletics),
            Some(Skill {
                modifier: 1,
                proficiency: Proficiency::Proficient
            })
        );
        assert_eq!(
            character(Some(1), Some(9), Proficiency::Proficient).skill(SkillName::Athletics),
            Some(Skill {
                modifier: 1,
                proficiency: Proficiency::Proficient
            })
        );
        assert_eq!(
            character(Some(1), Some(10), Proficiency::Proficient).skill(SkillName::Athletics),
            Some(Skill {
                modifier: 2,
                proficiency: Proficiency::Proficient
            })
        );
        assert_eq!(
            character(Some(1), Some(11), Proficiency::Proficient).skill(SkillName::Athletics),
            Some(Skill {
                modifier: 2,
                proficiency: Proficiency::Proficient
            })
        );
        assert_eq!(
            character(Some(1), Some(12), Proficiency::Proficient).skill(SkillName::Athletics),
            Some(Skill {
                modifier: 3,
                proficiency: Proficiency::Proficient
            })
        );
        assert_eq!(
            character(Some(1), Some(13), Proficiency::Proficient).skill(SkillName::Athletics),
            Some(Skill {
                modifier: 3,
                proficiency: Proficiency::Proficient
            })
        );
        assert_eq!(
            character(Some(1), Some(29), Proficiency::Proficient).skill(SkillName::Athletics),
            Some(Skill {
                modifier: 11,
                proficiency: Proficiency::Proficient
            })
        );
        assert_eq!(
            character(Some(1), Some(30), Proficiency::Proficient).skill(SkillName::Athletics),
            Some(Skill {
                modifier: 12,
                proficiency: Proficiency::Proficient
            })
        );
    }
}
