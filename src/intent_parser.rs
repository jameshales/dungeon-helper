use crate::character::{
    AbilityName, CharacterAttribute, CharacterAttributeUpdate, Proficiency, SkillName,
};
use crate::character_roll::{CharacterRoll, Check};
use crate::command;
use crate::command::Command;
use crate::roll::{Condition, Error, Roll};
use snips_nlu_ontology::{IntentParserResult, Slot, SlotValue};
use std::convert::TryFrom;

pub fn parse_intent_result(result: &IntentParserResult) -> Option<Command> {
    let IntentParserResult { intent, slots, .. } = result;
    intent
        .intent_name
        .as_ref()
        .map(|intent_name| match intent_name.as_ref() {
            "rollAbility" => parse_roll_ability(&slots),
            "rollDice" => parse_roll_dice(&slots),
            "rollInitiative" => parse_roll_initiative(&slots),
            "rollSavingThrow" => parse_roll_saving_throw(&slots),
            "rollSkill" => parse_roll_skill(&slots),
            "setAbility" => parse_set_ability(&slots),
            "setJackOfAllTrades" => Command::Set(CharacterAttributeUpdate::JackOfAllTrades(true)),
            "setLevel" => parse_set_level(&slots),
            "setNotJackOfAllTrades" => {
                Command::Set(CharacterAttributeUpdate::JackOfAllTrades(false))
            }
            "setSavingThrow" => parse_set_saving_throw(&slots),
            "setSkill" => parse_set_skill(&slots),
            "setBotDisabled" => Command::SetBotDisabled,
            "setBotEnabled" => Command::SetBotEnabled,
            "setCharactersLocked" => Command::SetCharactersLocked,
            "setCharactersUnlocked" => Command::SetCharactersUnlocked,
            "showAbilities" => Command::ShowAbilities,
            "showAbility" => parse_show_ability(&slots),
            "showHelp" => Command::Help,
            "showInitiative" => Command::Show(CharacterAttribute::Initiative),
            "showJackOfAllTrades" => Command::Show(CharacterAttribute::JackOfAllTrades),
            "showLevel" => Command::Show(CharacterAttribute::Level),
            "showPassiveAbility" => parse_show_passive_ability(&slots),
            "showPassiveSkill" => parse_show_passive_skill(&slots),
            "showSavingThrow" => parse_show_saving_throw(&slots),
            "showSkill" => parse_show_skill(&slots),
            "showSkills" => Command::ShowSkills,
            intent_name => {
                Command::ShowError(command::Error::UnknownIntent(intent_name.to_string()))
            }
        })
}

fn parse_roll_ability(slots: &Vec<Slot>) -> Command {
    let ability = extract_ability_slot(slots);
    let condition = extract_condition_slot(slots).unwrap_or(Condition::Normal);
    ability.map_or(
        Command::Clarification(format!("It looks like you're trying to roll an ability check, but I'm not sure which ability you want. Try \"Roll strength\", \"Dexterity check\", etc.")),
        |ability| {
            let roll = CharacterRoll {
                check: Check::Ability(ability),
                condition
            };
            Command::CharacterRoll(roll)
        }
    )
}

fn parse_roll_dice(slots: &Vec<Slot>) -> Command {
    let condition = extract_condition_slot(slots).unwrap_or(Condition::Normal);
    let rolls = extract_usize_slot_value(slots, "rolls").unwrap_or(1);
    let sides = extract_die_slot(slots);
    sides.map_or(
        Command::Clarification(format!("It looks like you're trying to roll some dice, but I'm not sure what kind of dice you want. Try \"Roll a d20\", \"Throw two four-sided dice\", etc.")),
        |sides| {
            match Roll::new(rolls, sides, 0, condition) {
                Ok(roll) => Command::Roll(roll),
                Err(Error::RollsNonPositive) => {
                    Command::Clarification(format!("It looks like you're trying to roll {} dice. I can only roll a positive number of dice. Try rolling one or more dice.", rolls))
                }
                Err(Error::RollsTooGreat) => {
                    Command::Clarification(format!("It looks like you're trying to roll {} dice. That's too many dice! Try rolling 100 or fewer dice.", rolls))
                }
                Err(Error::SidesNonPositive) => {
                    Command::Clarification(format!("It looks like you're trying to roll dice with {} sides. I can only roll a positive number of sides. Try rolling dice with one or more sides.", sides))
                }
                Err(Error::SidesTooGreat) => {
                    Command::Clarification(format!("It looks like you're trying to roll dice with {} sides. That's too many sides! Try rolling dice with 100 or fewer sides.", sides))
                }
            }
        }
    )
}

fn parse_roll_initiative(slots: &Vec<Slot>) -> Command {
    let condition = extract_condition_slot(slots).unwrap_or(Condition::Normal);
    let roll = CharacterRoll {
        check: Check::Initiative,
        condition,
    };
    Command::CharacterRoll(roll)
}

fn parse_roll_saving_throw(slots: &Vec<Slot>) -> Command {
    let ability = extract_ability_slot(slots);
    let condition = extract_condition_slot(slots).unwrap_or(Condition::Normal);
    ability.map_or(
        Command::Clarification(format!("It looks like you're trying to roll a saving throw, but I'm not sure what kind of saving throw you want. Try \"Roll strength saving throw\", \"Dexterity saving throw\", etc.")),
        |ability| {
            let roll = CharacterRoll {
                check: Check::SavingThrow(ability),
                condition
            };
            Command::CharacterRoll(roll)
        }
    )
}

fn parse_roll_skill(slots: &Vec<Slot>) -> Command {
    let condition = extract_condition_slot(slots).unwrap_or(Condition::Normal);
    let skill = extract_skill_slot(slots);
    skill.map_or(
        Command::Clarification(format!("It looks like you're trying to roll a skill check, but I'm not sure what skill you want. Try \"Roll stealth\", \"Athletics check\", etc.")),
        |skill| {
            let roll = CharacterRoll {
                check: Check::Skill(skill),
                condition
            };
            Command::CharacterRoll(roll)
        }
    )
}

fn parse_set_ability(slots: &Vec<Slot>) -> Command {
    let ability = extract_ability_slot(slots);
    let score = extract_i32_slot_value(slots, "score");
    ability.map_or(
        Command::Clarification(format!("It looks like you're trying to set an ability score, but I'm not sure what ability you want to set. Try \"Set strength as 12\", \"Change dexterity to 14\", etc.")),
        |ability| {
            score.map_or(
                Command::Clarification(format!("It looks like you're trying to set an ability score, but I'm not sure what score you want to set it to. Try \"Set strength as 12\", \"Change dexterity to 14\", etc.")),
                |score| {
                    Command::Set(CharacterAttributeUpdate::Ability(ability, score))
                }
            )
        }
    )
}

fn parse_set_level(slots: &Vec<Slot>) -> Command {
    let level = extract_i32_slot_value(slots, "level");
    level.map_or(
        Command::Clarification(format!("It looks like you're trying to set your level, but I'm not sure what level you want to set it to. Try \"Set level as 3\", \"Change level to 5\", etc.")),
        |level| {
            Command::Set(CharacterAttributeUpdate::Level(level))
        }
    )
}

fn parse_set_saving_throw(slots: &Vec<Slot>) -> Command {
    let ability = extract_ability_slot(slots);
    let proficiency = extract_proficiency_slot(slots);
    ability.map_or(
        Command::Clarification(format!("It looks like you're trying to set a saving throw proficiency, but I'm not sure what saving throw you want to set. Try \"Set strength saving throw to proficient\", \"Change dexterity saving throw to normal\", etc.")),
        |ability| {
            proficiency.map_or(
                Command::Clarification(format!("It looks like you're trying to set a saving throw proficiency, but I'm not sure what proficiency you want to set it to. Try \"Set strength saving throw to proficient\", \"Change dexterity saving throw to normal\", etc.")),
                |proficiency| {
                    Command::Set(CharacterAttributeUpdate::SavingThrowProficiency(ability, proficiency != Proficiency::Normal))
                }
            )
        }
    )
}

fn parse_set_skill(slots: &Vec<Slot>) -> Command {
    let proficiency = extract_proficiency_slot(slots);
    let skill = extract_skill_slot(slots);
    skill.map_or(
        Command::Clarification(format!("It looks like you're trying to set a skill proficiency, but I'm not sure what skill you want to set. Try \"Set athletics to proficient\", \"Change stealth to expert\", \"Update nature to normal\" etc.")),
        |skill| {
            proficiency.map_or(
                Command::Clarification(format!("It looks like you're trying to set a skill proficiency, but I'm not sure what proficiency you want to set it to. Try \"Set athletics to proficient\", \"Change stealth to expert\", \"Update nature to normal\" etc.")),
                |proficiency| {
                    Command::Set(CharacterAttributeUpdate::SkillProficiency(skill, proficiency))
                }
            )
        }
    )
}

fn parse_show_ability(slots: &Vec<Slot>) -> Command {
    let ability = extract_ability_slot(slots);
    ability.map_or(
        Command::Clarification(format!("It looks like you're trying to view an ability score, but I'm not sure what ability you want. Try \"Show strength\", \"Display dexterity\", etc.")),
        |ability| {
            Command::Show(CharacterAttribute::Ability(ability))
        }
    )
}

fn parse_show_passive_ability(slots: &Vec<Slot>) -> Command {
    let ability = extract_ability_slot(slots);
    ability.map_or(
        Command::Clarification(format!("It looks like you're trying to view a passive ability score, but I'm not sure what ability you want. Try \"Show passive strength\", \"Display passive dexterity\", etc.")),
        |ability| {
            Command::Show(CharacterAttribute::PassiveAbility(ability))
        }
    )
}

fn parse_show_passive_skill(slots: &Vec<Slot>) -> Command {
    let skill = extract_skill_slot(slots);
    skill.map_or(
        Command::Clarification(format!("It looks like you're trying to view a passive skill score, but I'm not sure what skill you want. Try \"Show passive athletics\", \"Display passive stealth\", etc.")),
        |skill| {
            Command::Show(CharacterAttribute::PassiveSkill(skill))
        }
    )
}

fn parse_show_saving_throw(slots: &Vec<Slot>) -> Command {
    let ability = extract_ability_slot(slots);
    ability.map_or(
        Command::Clarification(format!("It looks like you're trying to view a saving throw modifier, but I'm not sure what ability you want. Try \"Show strength saving throw\", \"Display passive saving throw\", etc.")),
        |ability| {
            Command::Show(CharacterAttribute::SavingThrow(ability))
        }
    )
}

fn parse_show_skill(slots: &Vec<Slot>) -> Command {
    let skill = extract_skill_slot(slots);
    skill.map_or(
        Command::Clarification(format!("It looks like you're trying to view a skill modifier, but I'm not sure what skill you want. Try \"Show athletics\", \"Display stealth\", etc.")),
        |skill| {
            Command::Show(CharacterAttribute::Skill(skill))
        }
    )
}

fn extract_ability_slot(slots: &Vec<Slot>) -> Option<AbilityName> {
    extract_custom_slot_value(slots, "ability").and_then(|value| AbilityName::parse(value.as_ref()))
}

fn extract_condition_slot(slots: &Vec<Slot>) -> Option<Condition> {
    extract_custom_slot_value(slots, "condition").and_then(|value| match value.as_ref() {
        "advantage" => Some(Condition::Advantage),
        "disadvantage" => Some(Condition::Disadvantage),
        "normal" => Some(Condition::Normal),
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
