use crate::character::{AbilityName, Character, SkillName};
use crate::roll::{Condition, Roll};
use regex::Regex;
use std::fmt;

pub struct CharacterRoll {
    pub check: Check,
    pub condition: Condition,
}

impl CharacterRoll {
    pub fn parse(string: &str) -> Option<CharacterRoll> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(.*?)(?: with (advantage|disadvantage))?$").unwrap();
        }

        RE.captures(string).and_then(|captures| {
            let check = captures.get(1).and_then(|m| Check::parse(m.as_str()))?;
            let condition = match captures.get(2).map(|m| m.as_str()) {
                Some("advantage") => Condition::Advantage,
                Some("disadvantage") => Condition::Disadvantage,
                _ => Condition::Normal,
            };
            Some(CharacterRoll { check, condition })
        })
    }

    pub fn to_roll(&self, character: &Character) -> Option<Roll> {
        let modifier = match self.check {
            Check::Ability(name) => character.ability(name)?.modifier,
            Check::Initiative => character.ability(AbilityName::Dexterity)?.modifier,
            Check::SavingThrow(name) => character.saving_throw(name)?.modifier,
            Check::Skill(name) => character.skill(name)?.modifier,
        };
        Some(Roll::new(1, 20, modifier, self.condition).unwrap())
    }
}

pub enum Check {
    Ability(AbilityName),
    Initiative,
    SavingThrow(AbilityName),
    Skill(SkillName),
}

impl Check {
    pub fn parse(string: &str) -> Option<Check> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(.*) saving throw$").unwrap();
        }
        AbilityName::parse(string)
            .map(Check::Ability)
            .or(if string.to_lowercase() == "initiative" {
                Some(Check::Initiative)
            } else {
                None
            })
            .or(SkillName::parse(string).map(Check::Skill))
            .or(RE
                .captures(string)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str())
                .and_then(AbilityName::parse)
                .map(Check::SavingThrow))
    }
}

impl fmt::Display for Check {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Check::Ability(name) => write!(f, "{}", name.as_str()),
            Check::Initiative => write!(f, "Initiative"),
            Check::SavingThrow(name) => write!(f, "{} saving throw", name.as_str()),
            Check::Skill(name) => write!(f, "{}", name.as_str()),
        }
    }
}
