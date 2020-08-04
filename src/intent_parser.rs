use crate::attack_roll::{
    AttackRoll, Handedness, ImprovisedWeaponAttackRoll, UnarmedStrikeAttackRoll, WeaponAttackRoll,
};
use crate::character::{AbilityName, SkillName};
use crate::character_roll::{CharacterRoll, Check};
use crate::command::{Command, Error};
use crate::roll::{Condition, ConditionalRoll};
use crate::weapon::{AmbiguousWeaponName, Classification, WeaponName};
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
            "showHelp" => Ok(Command::Help),
            intent_name => Err(Error::UnknownIntent(intent_name.to_owned())),
        })
}

fn parse_roll_ability(slots: &[Slot]) -> Result<Command, Error> {
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

fn parse_roll_attack(slots: &[Slot]) -> Result<Command, Error> {
    let ambiguous_weapon = extract_ambiguous_weapon_slot(slots);
    let classification = extract_classification_slot(slots);
    let condition = extract_condition_slot(slots);
    let handedness = extract_handedness_slot(slots);
    let improvised_weapon = extract_improvised_weapon_slot(slots);
    let weapon = extract_weapon_slot(slots);
    weapon
        .ok_or_else(|| {
            ambiguous_weapon.map_or(
                Error::RollAttackMissingWeapon,
                Error::RollAttackAmbiguousWeapon,
            )
        })
        .and_then(|weapon| {
            if weapon.to_weapon().versatile.is_some() && handedness.is_none() {
                Err(Error::RollAttackMissingHandedness)
            } else {
                Ok(AttackRoll::Weapon(WeaponAttackRoll {
                    weapon,
                    classification,
                    condition,
                    handedness,
                }))
            }
        })
        .or_else(|error| {
            if improvised_weapon {
                classification
                    .ok_or(Error::RollAttackMissingClassification)
                    .map(|classification| {
                        AttackRoll::ImprovisedWeapon(ImprovisedWeaponAttackRoll {
                            classification,
                            condition,
                        })
                    })
            } else {
                Err(error)
            }
        })
        .map(Command::AttackRoll)
}

fn parse_roll_dice(slots: &[Slot]) -> Result<Command, Error> {
    let condition = extract_condition_slot(slots);
    let rolls = extract_usize_slot_value(slots, "rolls").unwrap_or(1);
    let sides = extract_die_slot(slots);
    sides.ok_or(Error::RollDiceMissingSides).and_then(|sides| {
        ConditionalRoll::new(rolls, sides, 0, condition)
            .map(Command::Roll)
            .map_err(|error| Error::RollDiceInvalid(error, rolls, sides))
    })
}

fn parse_roll_initiative(slots: &[Slot]) -> Command {
    let condition = extract_condition_slot(slots);
    let roll = CharacterRoll {
        check: Check::Initiative,
        condition,
    };
    Command::CharacterRoll(roll)
}

fn parse_roll_saving_throw(slots: &[Slot]) -> Result<Command, Error> {
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

fn parse_roll_skill(slots: &[Slot]) -> Result<Command, Error> {
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

fn parse_roll_unarmed_strike(slots: &[Slot]) -> Command {
    let condition = extract_condition_slot(slots);
    let roll = AttackRoll::UnarmedStrike(UnarmedStrikeAttackRoll { condition });
    Command::AttackRoll(roll)
}

fn extract_ability_slot(slots: &[Slot]) -> Option<AbilityName> {
    extract_custom_slot_value(slots, "ability").and_then(|value| AbilityName::parse(value.as_ref()))
}

fn extract_ambiguous_weapon_slot(slots: &[Slot]) -> Option<AmbiguousWeaponName> {
    extract_custom_slot_value(slots, "weapon")
        .and_then(|value| AmbiguousWeaponName::parse(value.as_ref()))
}

fn extract_classification_slot(slots: &[Slot]) -> Option<Classification> {
    extract_custom_slot_value(slots, "weapon_classification")
        .and_then(|value| Classification::parse(value.as_ref()))
}

fn extract_condition_slot(slots: &[Slot]) -> Option<Condition> {
    extract_custom_slot_value(slots, "condition").and_then(|value| match value.as_ref() {
        "advantage" => Some(Condition::Advantage),
        "disadvantage" => Some(Condition::Disadvantage),
        _ => None,
    })
}

fn extract_custom_slot_value<'a>(slots: &'a [Slot], slot_name: &str) -> Option<&'a String> {
    find_slot_by_name(slots, slot_name).and_then(|slot| match &slot.value {
        SlotValue::Custom(string_value) => Some(&string_value.value),
        _ => None,
    })
}

fn extract_die_slot(slots: &[Slot]) -> Option<i32> {
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

fn extract_handedness_slot(slots: &[Slot]) -> Option<Handedness> {
    extract_custom_slot_value(slots, "handedness")
        .and_then(|value| Handedness::parse(value.as_ref()))
}

fn extract_usize_slot_value<'a>(slots: &'a [Slot], slot_name: &str) -> Option<usize> {
    extract_f64_slot_value(slots, slot_name).and_then(|v| usize::try_from(v as i64).ok())
}

fn extract_f64_slot_value<'a>(slots: &'a [Slot], slot_name: &str) -> Option<f64> {
    slots
        .iter()
        .find(|slot| slot.slot_name == slot_name)
        .and_then(|slot| match &slot.value {
            SlotValue::Number(number_value) => Some(number_value.value),
            _ => None,
        })
}

fn extract_improvised_weapon_slot(slots: &[Slot]) -> bool {
    extract_custom_slot_value(slots, "weapon").map_or(false, |v| v == "improvised weapon")
}

fn extract_skill_slot(slots: &[Slot]) -> Option<SkillName> {
    extract_custom_slot_value(slots, "skill").and_then(|value| SkillName::parse(value.as_ref()))
}

fn extract_weapon_slot(slots: &[Slot]) -> Option<WeaponName> {
    extract_custom_slot_value(slots, "weapon").and_then(|value| WeaponName::parse(value.as_ref()))
}

fn find_slot_by_name<'a>(slots: &'a [Slot], slot_name: &str) -> Option<&'a Slot> {
    slots.iter().find(|slot| slot.slot_name == slot_name)
}
