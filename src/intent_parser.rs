use crate::attack_roll::{AttackRoll, Handedness, ImprovisedWeaponAttackRoll, UnarmedStrikeAttackRoll, WeaponAttackRoll};
use crate::character::{
    AbilityName, CharacterAttribute, CharacterAttributeUpdate, Proficiency, SkillName,
};
use crate::character_roll::{CharacterRoll, Check};
use crate::command::{Command, Error};
use crate::roll::{Condition, ConditionalRoll};
use crate::weapon::{AmbiguousWeaponName, Category, Classification, WeaponName};
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
            "rollAttack" => parse_roll_attack(&slots),
            "rollDice" => parse_roll_dice(&slots),
            "rollInitiative" => Ok(parse_roll_initiative(&slots)),
            "rollSavingThrow" => parse_roll_saving_throw(&slots),
            "rollSkill" => parse_roll_skill(&slots),
            "rollUnarmedStrike" => Ok(parse_roll_unarmed_strike(&slots)),
            "setAbility" => parse_set_ability(&slots),
            "setChannelDiceOnlyFalse" => Ok(Command::SetChannelDiceOnly(false)),
            "setChannelDiceOnlyTrue" => Ok(Command::SetChannelDiceOnly(true)),
            "setChannelEnabledFalse" => Ok(Command::SetChannelEnabled(false)),
            "setChannelEnabledTrue" => Ok(Command::SetChannelEnabled(true)),
            "setChannelLockedFalse" => Ok(Command::SetChannelLocked(false)),
            "setChannelLockedTrue" => Ok(Command::SetChannelLocked(true)),
            "setJackOfAllTrades" => Ok(Command::Set(CharacterAttributeUpdate::JackOfAllTrades(
                true,
            ))),
            "setLevel" => parse_set_level(&slots),
            "setNotJackOfAllTrades" => Ok(Command::Set(CharacterAttributeUpdate::JackOfAllTrades(
                false,
            ))),
            "setSavingThrow" => parse_set_saving_throw(&slots),
            "setSkill" => parse_set_skill(&slots),
            "setWeaponProficiency" => parse_set_weapon_proficiency(&slots),
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
            "showWeaponProficiencies" => Ok(Command::ShowWeaponProficiencies),
            intent_name => Err(Error::UnknownIntent(intent_name.to_owned())),
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

fn parse_roll_attack(slots: &Vec<Slot>) -> Result<Command, Error> {
    let ambiguous_weapon = extract_ambiguous_weapon_slot(slots);
    let classification = extract_classification_slot(slots);
    let condition = extract_condition_slot(slots);
    let handedness = extract_handedness_slot(slots);
    let improvised_weapon = extract_improvised_weapon_slot(slots);
    let weapon = extract_weapon_slot(slots);
    weapon
        .ok_or(ambiguous_weapon.map_or(
            Error::RollAttackMissingWeapon,
            Error::RollAttackAmbiguousWeapon,
        ))
        .and_then(|weapon| {
            if weapon.to_weapon().versatile.is_some() && handedness.is_none() {
                Err(Error::RollAttackMissingHandedness)
            } else {
                Ok(AttackRoll::Weapon(WeaponAttackRoll {
                    weapon,
                    classification,
                    condition,
                    handedness
                }))
            }
        })
        .or_else(|error|
            if improvised_weapon {
                classification.ok_or(Error::RollAttackMissingClassification)
                    .map(|classification|
                        AttackRoll::ImprovisedWeapon(ImprovisedWeaponAttackRoll {
                            classification,
                            condition,
                        })
                    )
            } else {
                Err(error)
            }
        )
        .map(Command::AttackRoll)
}

fn parse_roll_dice(slots: &Vec<Slot>) -> Result<Command, Error> {
    let condition = extract_condition_slot(slots);
    let rolls = extract_usize_slot_value(slots, "rolls").unwrap_or(1);
    let sides = extract_die_slot(slots);
    sides.ok_or(Error::RollDiceMissingSides).and_then(|sides| {
        ConditionalRoll::new(rolls, sides, 0, condition)
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

fn parse_roll_unarmed_strike(slots: &Vec<Slot>) -> Command {
    let condition = extract_condition_slot(slots);
    let roll = AttackRoll::UnarmedStrike(UnarmedStrikeAttackRoll {
        condition,
    });
    Command::AttackRoll(roll)
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

fn parse_set_weapon_proficiency(slots: &Vec<Slot>) -> Result<Command, Error> {
    let ambiguous_weapon = extract_ambiguous_weapon_slot(slots);
    let proficiency = extract_proficiency_slot(slots);
    let weapon = extract_weapon_slot(slots);
    let weapon_proficiency = extract_weapon_proficiency_slot(slots);
    proficiency
        .ok_or(Error::SetWeaponProficiencyMissingProficiency)
        .and_then(|proficiency| {
            let proficient = proficiency != Proficiency::Normal;
            weapon
                .map(|weapon| {
                    Command::Set(CharacterAttributeUpdate::WeaponProficiency(
                        weapon, proficient,
                    ))
                })
                .or(weapon_proficiency.map(|weapon_proficiency| {
                    Command::Set(CharacterAttributeUpdate::WeaponCategoryProficiency(
                        weapon_proficiency,
                        proficient,
                    ))
                }))
                .ok_or(ambiguous_weapon.map_or(
                    Error::SetWeaponProficiencyMissingWeaponAndCategory,
                    Error::SetWeaponProficiencyAmbiguousWeapon,
                ))
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

fn extract_ambiguous_weapon_slot(slots: &Vec<Slot>) -> Option<AmbiguousWeaponName> {
    extract_custom_slot_value(slots, "weapon")
        .and_then(|value| AmbiguousWeaponName::parse(value.as_ref()))
}

fn extract_classification_slot(slots: &Vec<Slot>) -> Option<Classification> {
    extract_custom_slot_value(slots, "weapon_classification")
        .and_then(|value| Classification::parse(value.as_ref()))
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

fn extract_handedness_slot(slots: &Vec<Slot>) -> Option<Handedness> {
    extract_custom_slot_value(slots, "handedness")
        .and_then(|value| Handedness::parse(value.as_ref()))
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

fn extract_improvised_weapon_slot(slots: &Vec<Slot>) -> bool {
    extract_custom_slot_value(slots, "weapon").map_or(false, |v| v == "improvised weapon")
}

fn extract_proficiency_slot(slots: &Vec<Slot>) -> Option<Proficiency> {
    extract_custom_slot_value(slots, "proficiency")
        .and_then(|value| Proficiency::parse(value.as_ref()))
}

fn extract_skill_slot(slots: &Vec<Slot>) -> Option<SkillName> {
    extract_custom_slot_value(slots, "skill").and_then(|value| SkillName::parse(value.as_ref()))
}

fn extract_weapon_slot(slots: &Vec<Slot>) -> Option<WeaponName> {
    extract_custom_slot_value(slots, "weapon").and_then(|value| WeaponName::parse(value.as_ref()))
}

fn extract_weapon_proficiency_slot(slots: &Vec<Slot>) -> Option<Category> {
    extract_custom_slot_value(slots, "weapon_proficiency")
        .and_then(|value| Category::parse(value.as_ref()))
}

fn find_slot_by_name<'a>(slots: &'a Vec<Slot>, slot_name: &str) -> Option<&'a Slot> {
    slots.iter().find(|slot| slot.slot_name == slot_name)
}
