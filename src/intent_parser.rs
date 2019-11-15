use crate::character::{
    AbilityName, CharacterAttribute, CharacterAttributeUpdate, Proficiency, SkillName,
};
use crate::character_roll::{CharacterRoll, Check};
use crate::command::{Command, Error};
use crate::roll::{Condition, Roll};
use snips_nlu_ontology::{IntentParserResult, Slot, SlotValue};
use std::convert::TryFrom;

pub fn parse_intent_result(result: &IntentParserResult) -> Result<Command, Error> {
    let IntentParserResult { intent, slots, .. } = result;
    intent
        .intent_name
        .as_ref()
        .ok_or(Error::NoIntent)
        .and_then(|intent_name| match intent_name.as_ref() {
            "rollAbility" => parse_roll_ability(&slots),
            "rollDice" => parse_roll_dice(&slots),
            "rollInitiative" => Ok(parse_roll_initiative(&slots)),
            "rollSavingThrow" => parse_roll_saving_throw(&slots),
            "rollSkill" => parse_roll_skill(&slots),
            "setAbility" => parse_set_ability(&slots),
            "setChannelDiceOnlyFalse" => Ok(Command::SetChannelDiceOnly(false)),
            "setChannelDiceOnlyTrue" => Ok(Command::SetChannelDiceOnly(true)),
            "setChannelEnabledFalse" => Ok(Command::SetChannelEnabled(false)),
            "setChannelEnabledTrue" => Ok(Command::SetChannelEnabled(true)),
            "setChannelLockedFalse" => Ok(Command::SetChannelLocked(true)),
            "setChannelLockedTrue" => Ok(Command::SetChannelLocked(false)),
            "setJackOfAllTrades" => Ok(Command::Set(CharacterAttributeUpdate::JackOfAllTrades(
                true,
            ))),
            "setLevel" => parse_set_level(&slots),
            "setNotJackOfAllTrades" => Ok(Command::Set(CharacterAttributeUpdate::JackOfAllTrades(
                false,
            ))),
            "setSavingThrow" => parse_set_saving_throw(&slots),
            "setSkill" => parse_set_skill(&slots),
            "showAbilities" => Ok(Command::ShowAbilities),
            "showAbility" => parse_show_ability(&slots),
            "showHelp" => Ok(Command::Help),
            "showInitiative" => Ok(Command::Show(CharacterAttribute::Initiative)),
            "showJackOfAllTrades" => Ok(Command::Show(CharacterAttribute::JackOfAllTrades)),
            "showLevel" => Ok(Command::Show(CharacterAttribute::Level)),
            "showPassiveAbility" => parse_show_passive_ability(&slots),
            "showPassiveSkill" => parse_show_passive_skill(&slots),
            "showSavingThrow" => parse_show_saving_throw(&slots),
            "showSkill" => parse_show_skill(&slots),
            "showSkills" => Ok(Command::ShowSkills),
            intent_name => Err(Error::UnknownIntent(intent_name.to_string())),
        })
}

fn parse_roll_ability(slots: &Vec<Slot>) -> Result<Command, Error> {
    let ability = extract_ability_slot(slots);
    let condition = extract_condition_slot(slots);
    ability
        .ok_or(Error::RollAbilityMissingAbility)
        .map(|ability| {
            let roll = CharacterRoll {
                check: Check::Ability(ability),
                condition,
            };
            Command::CharacterRoll(roll)
        })
}

fn parse_roll_dice(slots: &Vec<Slot>) -> Result<Command, Error> {
    let condition = extract_condition_slot(slots);
    let rolls = extract_usize_slot_value(slots, "rolls").unwrap_or(1);
    let sides = extract_die_slot(slots);
    sides.ok_or(Error::RollDiceMissingSides).and_then(|sides| {
        Roll::new(rolls, sides, 0, condition)
            .map(Command::Roll)
            .map_err(|error| Error::RollDiceInvalid(error, rolls, sides))
    })
}

fn parse_roll_initiative(slots: &Vec<Slot>) -> Command {
    let condition = extract_condition_slot(slots);
    let roll = CharacterRoll {
        check: Check::Initiative,
        condition,
    };
    Command::CharacterRoll(roll)
}

fn parse_roll_saving_throw(slots: &Vec<Slot>) -> Result<Command, Error> {
    let ability = extract_ability_slot(slots);
    let condition = extract_condition_slot(slots);
    ability
        .ok_or(Error::RollSavingThrowMissingAbility)
        .map(|ability| {
            let roll = CharacterRoll {
                check: Check::SavingThrow(ability),
                condition,
            };
            Command::CharacterRoll(roll)
        })
}

fn parse_roll_skill(slots: &Vec<Slot>) -> Result<Command, Error> {
    let condition = extract_condition_slot(slots);
    let skill = extract_skill_slot(slots);
    skill.ok_or(Error::RollSkillMissingSkill).map(|skill| {
        let roll = CharacterRoll {
            check: Check::Skill(skill),
            condition,
        };
        Command::CharacterRoll(roll)
    })
}

fn parse_set_ability(slots: &Vec<Slot>) -> Result<Command, Error> {
    let ability = extract_ability_slot(slots);
    let score = extract_i32_slot_value(slots, "score");
    ability
        .ok_or(Error::SetAbilityMissingAbility)
        .and_then(|ability| {
            score
                .ok_or(Error::SetAbilityMissingScore)
                .map(|score| Command::Set(CharacterAttributeUpdate::Ability(ability, score)))
        })
}

fn parse_set_level(slots: &Vec<Slot>) -> Result<Command, Error> {
    let level = extract_i32_slot_value(slots, "level");
    level
        .ok_or(Error::SetLevelMissingLevel)
        .map(|level| Command::Set(CharacterAttributeUpdate::Level(level)))
}

fn parse_set_saving_throw(slots: &Vec<Slot>) -> Result<Command, Error> {
    let ability = extract_ability_slot(slots);
    let proficiency = extract_proficiency_slot(slots);
    ability
        .ok_or(Error::SetSavingThrowMissingAbility)
        .and_then(|ability| {
            proficiency
                .ok_or(Error::SetSavingThrowMissingProficiency)
                .map(|proficiency| {
                    Command::Set(CharacterAttributeUpdate::SavingThrowProficiency(
                        ability,
                        proficiency != Proficiency::Normal,
                    ))
                })
        })
}

fn parse_set_skill(slots: &Vec<Slot>) -> Result<Command, Error> {
    let proficiency = extract_proficiency_slot(slots);
    let skill = extract_skill_slot(slots);
    skill.ok_or(Error::SetSkillMissingSkill).and_then(|skill| {
        proficiency
            .ok_or(Error::SetSkillMissingProficiency)
            .map(|proficiency| {
                Command::Set(CharacterAttributeUpdate::SkillProficiency(
                    skill,
                    proficiency,
                ))
            })
    })
}

fn parse_show_ability(slots: &Vec<Slot>) -> Result<Command, Error> {
    let ability = extract_ability_slot(slots);
    ability
        .ok_or(Error::ShowAbilityMissingAbility)
        .map(|ability| Command::Show(CharacterAttribute::Ability(ability)))
}

fn parse_show_passive_ability(slots: &Vec<Slot>) -> Result<Command, Error> {
    let ability = extract_ability_slot(slots);
    ability
        .ok_or(Error::ShowPassiveAbilityMissingAbility)
        .map(|ability| Command::Show(CharacterAttribute::PassiveAbility(ability)))
}

fn parse_show_passive_skill(slots: &Vec<Slot>) -> Result<Command, Error> {
    let skill = extract_skill_slot(slots);
    skill
        .ok_or(Error::ShowPassiveSkillMissingSkill)
        .map(|skill| Command::Show(CharacterAttribute::PassiveSkill(skill)))
}

fn parse_show_saving_throw(slots: &Vec<Slot>) -> Result<Command, Error> {
    let ability = extract_ability_slot(slots);
    ability
        .ok_or(Error::ShowSavingThrowMissingAbility)
        .map(|ability| Command::Show(CharacterAttribute::SavingThrow(ability)))
}

fn parse_show_skill(slots: &Vec<Slot>) -> Result<Command, Error> {
    let skill = extract_skill_slot(slots);
    skill
        .ok_or(Error::ShowSkillMissingSkill)
        .map(|skill| Command::Show(CharacterAttribute::Skill(skill)))
}

fn extract_ability_slot(slots: &Vec<Slot>) -> Option<AbilityName> {
    extract_custom_slot_value(slots, "ability").and_then(|value| AbilityName::parse(value.as_ref()))
}

fn extract_condition_slot(slots: &Vec<Slot>) -> Option<Condition> {
    extract_custom_slot_value(slots, "condition").and_then(|value| match value.as_ref() {
        "advantage" => Some(Condition::Advantage),
        "disadvantage" => Some(Condition::Disadvantage),
        _ => None,
    })
}

fn extract_custom_slot_value<'a>(slots: &'a Vec<Slot>, slot_name: &str) -> Option<&'a String> {
    find_slot_by_name(slots, slot_name).and_then(|slot| match &slot.value {
        SlotValue::Custom(string_value) => Some(&string_value.value),
        _ => None,
    })
}

fn extract_die_slot(slots: &Vec<Slot>) -> Option<i32> {
    extract_custom_slot_value(slots, "die").and_then(|value| match value.as_ref() {
        "d100" => Some(100),
        "d20" => Some(20),
        "d12" => Some(12),
        "d10" => Some(10),
        "d8" => Some(8),
        "d6" => Some(6),
        "d4" => Some(4),
        _ => None,
    })
}

fn extract_i32_slot_value<'a>(slots: &'a Vec<Slot>, slot_name: &str) -> Option<i32> {
    extract_f64_slot_value(slots, slot_name).and_then(|v| i32::try_from(v as i64).ok())
}

fn extract_usize_slot_value<'a>(slots: &'a Vec<Slot>, slot_name: &str) -> Option<usize> {
    extract_f64_slot_value(slots, slot_name).and_then(|v| usize::try_from(v as i64).ok())
}

fn extract_f64_slot_value<'a>(slots: &'a Vec<Slot>, slot_name: &str) -> Option<f64> {
    slots
        .iter()
        .find(|slot| slot.slot_name == slot_name)
        .and_then(|slot| match &slot.value {
            SlotValue::Number(number_value) => Some(number_value.value),
            _ => None,
        })
}

fn extract_proficiency_slot(slots: &Vec<Slot>) -> Option<Proficiency> {
    extract_custom_slot_value(slots, "proficiency")
        .and_then(|value| Proficiency::parse(value.as_ref()))
}

fn extract_skill_slot(slots: &Vec<Slot>) -> Option<SkillName> {
    extract_custom_slot_value(slots, "skill").and_then(|value| SkillName::parse(value.as_ref()))
}

fn find_slot_by_name<'a>(slots: &'a Vec<Slot>, slot_name: &str) -> Option<&'a Slot> {
    slots.iter().find(|slot| slot.slot_name == slot_name)
}
