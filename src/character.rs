use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use rusqlite::Result as RusqliteResult;
use rusqlite::{Connection, Row};
use serenity::model::id::{ChannelId, UserId};
use std::error;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Proficiency {
    Normal,
    Proficient,
    Expert,
}

impl Proficiency {
    pub fn parse(string: &str) -> Option<Proficiency> {
        match string {
            "Normal" => Some(Proficiency::Normal),
            "Proficient" => Some(Proficiency::Proficient),
            "Expert" => Some(Proficiency::Expert),
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
            Proficiency::parse(string).ok_or(FromSqlError::Other(Box::new(
                InvalidProficiencyValueError {
                    value: string.to_string(),
                },
            )))
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

#[derive(Debug, Eq, PartialEq)]
pub struct Character {
    level: Option<i32>,
    jack_of_all_trades: bool,

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
    pub fn from_row(row: &Row) -> RusqliteResult<Character> {
        Result::Ok(Character {
            level: row.get("level")?,
            jack_of_all_trades: row.get("jack_of_all_trades")?,

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

    pub fn get(
        connection: &Connection,
        channel_id: &ChannelId,
        user_id: &UserId,
    ) -> RusqliteResult<Character> {
        connection.query_row(
            "SELECT \
             level, \
             jack_of_all_trades, \
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
            |row| Character::from_row(row),
        )
    }

    fn proficiency_bonus(&self) -> Option<i32> {
        self.level.map(|level| level / 4 + 2)
    }

    // Abilities

    fn make_ability(score: Option<i32>) -> Option<Ability> {
        Some(Ability {
            score: score?,
            modifier: (score? - 10) / 2,
        })
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

    // Saving Throws

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

    // Skills

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
        self.make_skill(self.intelligence(), self.medicine_proficiency)
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
}

pub struct Ability {
    pub score: i32,
    pub modifier: i32,
}

pub struct SavingThrow {
    pub modifier: i32,
    pub proficiency: bool,
}

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
