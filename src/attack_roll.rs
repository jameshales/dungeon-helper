use crate::roll::{Condition, ConditionalRoll, Roll};
use crate::weapon::{Classification, WeaponName};
use std::cmp::max;

#[derive(Debug)]
pub enum AttackRoll {
    ImprovisedWeapon(ImprovisedWeaponAttackRoll),
    UnarmedStrike(UnarmedStrikeAttackRoll),
    Weapon(WeaponAttackRoll),
}

impl AttackRoll {
    pub fn to_attack_roll(
        &self,
        strength: Option<i32>,
        dexterity: Option<i32>,
        proficiency_bonus: Option<i32>,
        proficiency: bool,
        martial_arts: bool,
    ) -> Option<ConditionalRoll> {
        match self {
            AttackRoll::ImprovisedWeapon(roll) => roll.to_attack_roll(strength, dexterity),
            AttackRoll::UnarmedStrike(roll) => {
                roll.to_attack_roll(strength, dexterity, proficiency_bonus, martial_arts)
            }
            AttackRoll::Weapon(roll) => roll.to_attack_roll(
                strength,
                dexterity,
                proficiency_bonus,
                proficiency,
                martial_arts,
            ),
        }
    }

    pub fn to_damage_roll(
        &self,
        strength: Option<i32>,
        dexterity: Option<i32>,
        critical_hit: bool,
        martial_arts_damage_die: Option<i32>,
    ) -> Option<Roll> {
        match self {
            AttackRoll::ImprovisedWeapon(roll) => {
                roll.to_damage_roll(strength, dexterity, critical_hit)
            }
            AttackRoll::UnarmedStrike(roll) => {
                roll.to_damage_roll(strength, dexterity, critical_hit, martial_arts_damage_die)
            }
            AttackRoll::Weapon(roll) => {
                roll.to_damage_roll(strength, dexterity, critical_hit, martial_arts_damage_die)
            }
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            AttackRoll::ImprovisedWeapon(ImprovisedWeaponAttackRoll { classification, .. }) => {
                format!("improvised weapon (as {})", classification)
            }
            AttackRoll::UnarmedStrike(_) => "unarmed strike".to_owned(),
            AttackRoll::Weapon(WeaponAttackRoll {
                classification: Some(classification),
                weapon,
                ..
            }) => format!("{} (as {})", weapon, classification),
            AttackRoll::Weapon(WeaponAttackRoll {
                classification: None,
                weapon,
                ..
            }) => weapon.to_string(),
        }
    }

    pub fn get_handedness(&self) -> Option<Handedness> {
        match self {
            AttackRoll::Weapon(WeaponAttackRoll {
                handedness: Some(handedness),
                weapon,
                ..
            }) if weapon.to_weapon().versatile.is_some() => Some(*handedness),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct ImprovisedWeaponAttackRoll {
    pub classification: Classification,
    pub condition: Option<Condition>,
}

impl ImprovisedWeaponAttackRoll {
    pub fn to_attack_roll(
        &self,
        strength: Option<i32>,
        dexterity: Option<i32>,
    ) -> Option<ConditionalRoll> {
        let modifier = self.modifier(strength, dexterity)?;
        Some(ConditionalRoll::new_unsafe(1, 20, modifier, self.condition))
    }

    pub fn to_damage_roll(
        &self,
        strength: Option<i32>,
        dexterity: Option<i32>,
        critical_hit: bool,
    ) -> Option<Roll> {
        let multiplier = critical_hit_multiplier(critical_hit);
        let modifier = self.modifier(strength, dexterity)?;
        Some(Roll::new_unsafe(multiplier, 4, modifier))
    }

    fn modifier(&self, strength: Option<i32>, dexterity: Option<i32>) -> Option<i32> {
        match self.classification {
            Classification::Melee => strength,
            Classification::Ranged => dexterity,
        }
    }
}

#[derive(Debug)]
pub struct UnarmedStrikeAttackRoll {
    pub condition: Option<Condition>,
}

impl UnarmedStrikeAttackRoll {
    pub fn to_attack_roll(
        &self,
        strength: Option<i32>,
        dexterity: Option<i32>,
        proficiency_bonus: Option<i32>,
        martial_arts: bool,
    ) -> Option<ConditionalRoll> {
        let bonus = if martial_arts {
            UnarmedStrikeAttackRoll::get_martial_arts_bonus(strength, dexterity)?
        } else {
            strength?
        };
        Some(ConditionalRoll::new_unsafe(
            1,
            20,
            bonus + proficiency_bonus?,
            self.condition,
        ))
    }

    pub fn to_damage_roll(
        &self,
        strength: Option<i32>,
        dexterity: Option<i32>,
        critical_hit: bool,
        martial_arts_damage_die: Option<i32>,
    ) -> Option<Roll> {
        match martial_arts_damage_die {
            Some(martial_arts_damage_die) => {
                let multiplier = critical_hit_multiplier(critical_hit);
                let bonus = UnarmedStrikeAttackRoll::get_martial_arts_bonus(strength, dexterity)?;
                Some(Roll::new_unsafe(multiplier, martial_arts_damage_die, bonus))
            }
            None => Some(Roll::new_unsafe(0, 1, strength? + 1)),
        }
    }

    fn get_martial_arts_bonus(strength: Option<i32>, dexterity: Option<i32>) -> Option<i32> {
        match (strength, dexterity) {
            (Some(strength), Some(dexterity)) => Some(max(strength, dexterity)),
            (Some(strength), _) => Some(strength),
            (_, Some(dexterity)) => Some(dexterity),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct WeaponAttackRoll {
    pub weapon: WeaponName,
    pub classification: Option<Classification>,
    pub condition: Option<Condition>,
    pub handedness: Option<Handedness>,
}

impl WeaponAttackRoll {
    pub fn to_attack_roll(
        &self,
        strength: Option<i32>,
        dexterity: Option<i32>,
        proficiency_bonus: Option<i32>,
        proficiency: bool,
        martial_arts: bool,
    ) -> Option<ConditionalRoll> {
        let modifier = self.attack_modifier(
            strength,
            dexterity,
            proficiency_bonus,
            proficiency,
            martial_arts,
        );
        Some(ConditionalRoll::new_unsafe(
            1,
            20,
            modifier?,
            self.condition,
        ))
    }

    pub fn to_damage_roll(
        &self,
        strength: Option<i32>,
        dexterity: Option<i32>,
        critical_hit: bool,
        martial_arts_damage_die: Option<i32>,
    ) -> Option<Roll> {
        let weapon = self.weapon.to_weapon();
        let used_with_correct_classification = self.classification.iter().all(|c| {
            *c == weapon.classification || (*c == Classification::Ranged && weapon.thrown)
        });
        let roll = if used_with_correct_classification {
            let base = weapon
                .versatile
                .filter(|_| self.handedness == Some(Handedness::TwoHanded))
                .unwrap_or(weapon.damage);
            martial_arts_damage_die
                .filter(|_| weapon.is_monk_weapon())
                .map(|sides| {
                    Roll::new_clamped(base.rolls(), max(base.sides(), sides), base.modifier())
                })
                .unwrap_or(base)
        } else {
            Roll::new_clamped(1, 4, 0)
        };

        let multiplier = critical_hit_multiplier(critical_hit);
        let modifier = self.damage_modifier(strength, dexterity, martial_arts_damage_die.is_some());
        Some(roll.multiply_rolls(multiplier).add_modifier(modifier?))
    }

    fn attack_modifier(
        &self,
        strength: Option<i32>,
        dexterity: Option<i32>,
        proficiency_bonus: Option<i32>,
        proficiency: bool,
        martial_arts: bool,
    ) -> Option<i32> {
        let weapon = self.weapon.to_weapon();
        let proficiency_bonus =
            proficiency_bonus
                .map(|proficiency_bonus| if proficiency { proficiency_bonus } else { 0 });
        let modifier = match (self.classification.unwrap_or(weapon.classification), weapon.classification, weapon.thrown, weapon.finesse, martial_arts && weapon.is_monk_weapon()) {
            // Use a melee weapon as a melee weapon
            (Classification::Melee, Classification::Melee, _, false, false)
            // Use a thrown melee weapon as a ranged weapon
            | (Classification::Ranged, Classification::Melee, true, false, _) => strength? + proficiency_bonus?,
            // Use a ranged weapon as a ranged weapon
            (Classification::Ranged, Classification::Ranged, _, _, false) => dexterity? + proficiency_bonus?,
            // Use a monk weapon with strength or dexterity
            (Classification::Melee, Classification::Melee, _, _, true)
            | (Classification::Ranged, Classification::Ranged, _, _, true)
            // Use a melee weapon with finesse as a melee weapon
            | (Classification::Melee, Classification::Melee, _, true, false)
            // Use a thrown melee weapon with finesse as a ranged weapon
            | (Classification::Ranged, Classification::Melee, true, true, _) => max(strength?, dexterity?) + proficiency_bonus?,
            // Use a ranged weapon as a melee weapon (counts as improvised)
            (Classification::Melee, Classification::Ranged, _, _, _) => strength?,
            // Use a melee weapon as a ranged weapon (counts as improvised)
            (Classification::Ranged, Classification::Melee, false, _, _) => dexterity?,
        };
        Some(modifier)
    }

    fn damage_modifier(
        &self,
        strength: Option<i32>,
        dexterity: Option<i32>,
        martial_arts: bool,
    ) -> Option<i32> {
        let weapon = self.weapon.to_weapon();
        let modifier = match (self.classification.unwrap_or(weapon.classification), weapon.classification, weapon.thrown, weapon.finesse, martial_arts && weapon.is_monk_weapon()) {
            // Use a melee weapon as a melee weapon
            (Classification::Melee, Classification::Melee, _, false, false)
            // Use a thrown melee weapon as a ranged weapon
            | (Classification::Ranged, Classification::Melee, true, false, _) => strength?,
            // Use a ranged weapon as a ranged weapon
            (Classification::Ranged, Classification::Ranged, _, _, false) => dexterity?,
            // Use a monk weapon with strength or dexterity
            (Classification::Melee, Classification::Melee, _, _, true)
            | (Classification::Ranged, Classification::Ranged, _, _, true)
            // Use a melee weapon with finesse as a melee weapon
            | (Classification::Melee, Classification::Melee, _, true, false)
            // Use a thrown melee weapon with finesse as a ranged weapon
            | (Classification::Ranged, Classification::Melee, true, true, _) => max(strength?, dexterity?),
            // Use a ranged weapon as a melee weapon (counts as improvised)
            (Classification::Melee, Classification::Ranged, _, _, _) => strength?,
            // Use a melee weapon as a ranged weapon (counts as improvised)
            (Classification::Ranged, Classification::Melee, false, _, _) => dexterity?,
        };
        Some(modifier)
    }
}

fn critical_hit_multiplier(critical_hit: bool) -> usize {
    if critical_hit {
        2
    } else {
        1
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Handedness {
    OneHanded,
    TwoHanded,
}

impl Handedness {
    pub fn parse(string: &str) -> Option<Handedness> {
        match string.to_lowercase().as_str() {
            "one handed" => Some(Handedness::OneHanded),
            "two handed" => Some(Handedness::TwoHanded),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_improvised_melee_weapon_roll() {
        let roll = ImprovisedWeaponAttackRoll {
            classification: Classification::Melee,
            condition: None,
        };
        let strength = 2;
        let dexterity = 3;

        let expected_attack = Some(ConditionalRoll::new_unsafe(1, 20, 2, None));
        let expected_damage = Some(Roll::new_unsafe(1, 4, 2));

        let actual_attack = roll.to_attack_roll(Some(strength), Some(dexterity));
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false);

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_improvised_melee_weapon_roll_with_critical_hit() {
        let roll = ImprovisedWeaponAttackRoll {
            classification: Classification::Melee,
            condition: None,
        };
        let strength = 2;
        let dexterity = 3;

        let expected_damage = Some(Roll::new_unsafe(2, 4, 2));

        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), true);

        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_improvised_melee_weapon_roll_with_advantage() {
        let roll = ImprovisedWeaponAttackRoll {
            classification: Classification::Melee,
            condition: Some(Condition::Advantage),
        };
        let strength = 2;
        let dexterity = 3;

        let expected_attack = Some(ConditionalRoll::new_unsafe(
            1,
            20,
            2,
            Some(Condition::Advantage),
        ));
        let expected_damage = Some(Roll::new_unsafe(1, 4, 2));

        let actual_attack = roll.to_attack_roll(Some(strength), Some(dexterity));
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false);

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_improvised_melee_weapon_roll_with_disadvantage() {
        let roll = ImprovisedWeaponAttackRoll {
            classification: Classification::Melee,
            condition: Some(Condition::Disadvantage),
        };
        let strength = 2;
        let dexterity = 3;

        let expected_attack = Some(ConditionalRoll::new_unsafe(
            1,
            20,
            2,
            Some(Condition::Disadvantage),
        ));
        let expected_damage = Some(Roll::new_unsafe(1, 4, 2));

        let actual_attack = roll.to_attack_roll(Some(strength), Some(dexterity));
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false);

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_improvised_ranged_weapon_roll() {
        let roll = ImprovisedWeaponAttackRoll {
            classification: Classification::Ranged,
            condition: None,
        };
        let strength = 2;
        let dexterity = 3;

        let expected_attack = Some(ConditionalRoll::new_unsafe(1, 20, 3, None));
        let expected_damage = Some(Roll::new_unsafe(1, 4, 3));

        let actual_attack = roll.to_attack_roll(Some(strength), Some(dexterity));
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false);

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_improvised_ranged_weapon_roll_with_advantage() {
        let roll = ImprovisedWeaponAttackRoll {
            classification: Classification::Ranged,
            condition: Some(Condition::Advantage),
        };
        let strength = 2;
        let dexterity = 3;

        let expected_attack = Some(ConditionalRoll::new_unsafe(
            1,
            20,
            3,
            Some(Condition::Advantage),
        ));
        let expected_damage = Some(Roll::new_unsafe(1, 4, 3));

        let actual_attack = roll.to_attack_roll(Some(strength), Some(dexterity));
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false);

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_improvised_ranged_weapon_roll_with_disadvantage() {
        let roll = ImprovisedWeaponAttackRoll {
            classification: Classification::Ranged,
            condition: Some(Condition::Disadvantage),
        };
        let strength = 2;
        let dexterity = 3;

        let expected_attack = Some(ConditionalRoll::new_unsafe(
            1,
            20,
            3,
            Some(Condition::Disadvantage),
        ));
        let expected_damage = Some(Roll::new_unsafe(1, 4, 3));

        let actual_attack = roll.to_attack_roll(Some(strength), Some(dexterity));
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false);

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_unarmed_strike_roll() {
        let roll = UnarmedStrikeAttackRoll { condition: None };
        let strength = 2;
        let dexterity = 4;
        let proficiency_bonus = 3;

        let expected_attack = Some(ConditionalRoll::new_unsafe(1, 20, 5, None));
        let expected_damage = Some(Roll::new_unsafe(0, 1, 3));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            false,
        );
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false, None);

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_unarmed_strike_roll_with_critical_hit() {
        let roll = UnarmedStrikeAttackRoll { condition: None };
        let strength = 2;
        let dexterity = 4;

        let expected_damage = Some(Roll::new_unsafe(0, 1, 3));

        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), true, None);

        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_unarmed_strike_roll_with_advantage() {
        let roll = UnarmedStrikeAttackRoll {
            condition: Some(Condition::Advantage),
        };
        let strength = -1;
        let dexterity = 1;
        let proficiency_bonus = 3;

        let expected_attack = Some(ConditionalRoll::new_unsafe(
            1,
            20,
            2,
            Some(Condition::Advantage),
        ));
        let expected_damage = Some(Roll::new_unsafe(0, 1, 0));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            false,
        );
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false, None);

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_unarmed_strike_roll_with_disadvantage() {
        let roll = UnarmedStrikeAttackRoll {
            condition: Some(Condition::Disadvantage),
        };
        let strength = 2;
        let dexterity = 4;
        let proficiency_bonus = 1;

        let expected_attack = Some(ConditionalRoll::new_unsafe(
            1,
            20,
            3,
            Some(Condition::Disadvantage),
        ));
        let expected_damage = Some(Roll::new_unsafe(0, 1, 3));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            false,
        );
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false, None);

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_unarmed_strike_roll_without_strength() {
        let roll = UnarmedStrikeAttackRoll { condition: None };
        let dexterity = 2;
        let proficiency_bonus = 3;

        let expected_attack = None;
        let expected_damage = None;

        let actual_attack =
            roll.to_attack_roll(None, Some(dexterity), Some(proficiency_bonus), false);
        let actual_damage = roll.to_damage_roll(None, Some(dexterity), false, None);

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_unarmed_strike_roll_without_proficiency_bonus() {
        let roll = UnarmedStrikeAttackRoll { condition: None };
        let strength = 2;
        let dexterity = 4;

        let expected_attack = None;

        let actual_attack = roll.to_attack_roll(Some(strength), Some(dexterity), None, false);

        assert_eq!(actual_attack, expected_attack);
    }

    #[test]
    fn test_unarmed_strike_roll_with_martial_arts_both_higher() {
        let roll = UnarmedStrikeAttackRoll { condition: None };
        let strength = 2;
        let dexterity = 4;
        let proficiency_bonus = 3;

        let expected_attack = Some(ConditionalRoll::new_unsafe(1, 20, 7, None));
        let expected_damage = Some(Roll::new_unsafe(1, 6, 4));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            true,
        );
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false, Some(6));

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_unarmed_strike_roll_with_martial_arts_achen() {
        let roll = UnarmedStrikeAttackRoll { condition: None };
        let strength = 1;
        let dexterity = 4;
        let proficiency_bonus = 2;

        let expected_attack = Some(ConditionalRoll::new_unsafe(1, 20, 6, None));
        let expected_damage = Some(Roll::new_unsafe(1, 4, 4));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            true,
        );
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false, Some(4));

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_unarmed_strike_roll_without_martial_arts_connie() {
        let roll = UnarmedStrikeAttackRoll { condition: None };
        let strength = 2;
        let dexterity = -1;
        let proficiency_bonus = 2;

        let expected_attack = Some(ConditionalRoll::new_unsafe(1, 20, 4, None));
        let expected_damage = Some(Roll::new_unsafe(0, 1, 3));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            false,
        );
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false, None);

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_unarmed_strike_roll_with_martial_arts_loeguo() {
        let roll = UnarmedStrikeAttackRoll { condition: None };
        let strength = 3;
        let dexterity = -1;
        let proficiency_bonus = 2;

        let expected_attack = Some(ConditionalRoll::new_unsafe(1, 20, 5, None));
        let expected_damage = Some(Roll::new_unsafe(1, 4, 3));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            true,
        );
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false, Some(4));

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_unarmed_strike_roll_without_martial_arts_tocha() {
        let roll = UnarmedStrikeAttackRoll { condition: None };
        let strength = 2;
        let dexterity = 3;
        let proficiency_bonus = 2;

        let expected_attack = Some(ConditionalRoll::new_unsafe(1, 20, 4, None));
        let expected_damage = Some(Roll::new_unsafe(0, 1, 3));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            false,
        );
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false, None);

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_unarmed_strike_roll_with_martial_arts_dexterity_lower() {
        let roll = UnarmedStrikeAttackRoll { condition: None };
        let strength = 4;
        let dexterity = 2;
        let proficiency_bonus = 3;

        let expected_attack = Some(ConditionalRoll::new_unsafe(1, 20, 7, None));
        let expected_damage = Some(Roll::new_unsafe(1, 6, 4));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            true,
        );
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false, Some(6));

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_unarmed_strike_roll_with_martial_arts_damage_die_lower() {
        let roll = UnarmedStrikeAttackRoll { condition: None };
        let strength = 2;
        let dexterity = 4;
        let proficiency_bonus = 3;

        let expected_attack = Some(ConditionalRoll::new_unsafe(1, 20, 7, None));
        let expected_damage = Some(Roll::new_unsafe(1, 3, 4));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            true,
        );
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false, Some(3));

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_unarmed_strike_roll_with_martial_arts_damage_both_lower() {
        let roll = UnarmedStrikeAttackRoll { condition: None };
        let strength = 4;
        let dexterity = 2;
        let proficiency_bonus = 3;

        let expected_attack = Some(ConditionalRoll::new_unsafe(1, 20, 7, None));
        let expected_damage = Some(Roll::new_unsafe(1, 3, 4));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            true,
        );
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false, Some(3));

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_weapon_roll() {
        let roll = WeaponAttackRoll {
            weapon: WeaponName::Greatsword,
            classification: None,
            condition: None,
            handedness: None,
        };
        let strength = 2;
        let dexterity = 3;
        let proficiency_bonus = 3;

        let expected_attack = Some(ConditionalRoll::new_unsafe(1, 20, 2, None));
        let expected_damage = Some(Roll::new_unsafe(2, 6, 2));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            false,
            false,
        );
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false, None);

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_weapon_roll_with_critical_hit() {
        let roll = WeaponAttackRoll {
            weapon: WeaponName::Greatsword,
            classification: None,
            condition: None,
            handedness: None,
        };
        let strength = 2;
        let dexterity = 3;

        let expected_damage = Some(Roll::new_unsafe(4, 6, 2));

        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), true, None);

        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_weapon_roll_with_advantage() {
        let roll = WeaponAttackRoll {
            weapon: WeaponName::Greatsword,
            classification: None,
            condition: Some(Condition::Advantage),
            handedness: None,
        };
        let strength = 2;
        let dexterity = 3;
        let proficiency_bonus = 3;

        let expected_attack = Some(ConditionalRoll::new_unsafe(
            1,
            20,
            2,
            Some(Condition::Advantage),
        ));
        let expected_damage = Some(Roll::new_unsafe(2, 6, 2));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            false,
            false,
        );
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false, None);

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_weapon_roll_with_disadvantage() {
        let roll = WeaponAttackRoll {
            weapon: WeaponName::Greatsword,
            classification: None,
            condition: Some(Condition::Disadvantage),
            handedness: None,
        };
        let strength = 2;
        let dexterity = 3;
        let proficiency_bonus = 3;

        let expected_attack = Some(ConditionalRoll::new_unsafe(
            1,
            20,
            2,
            Some(Condition::Disadvantage),
        ));
        let expected_damage = Some(Roll::new_unsafe(2, 6, 2));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            false,
            false,
        );
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false, None);

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_weapon_roll_with_finesse() {
        let roll = WeaponAttackRoll {
            weapon: WeaponName::Rapier,
            classification: None,
            condition: None,
            handedness: None,
        };
        let strength = 2;
        let dexterity = 3;
        let proficiency_bonus = 3;

        let expected_attack = Some(ConditionalRoll::new_unsafe(1, 20, 3, None));
        let expected_damage = Some(Roll::new_unsafe(1, 8, 3));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            false,
            false,
        );
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false, None);

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_weapon_roll_with_thrown() {
        let roll = WeaponAttackRoll {
            weapon: WeaponName::Spear,
            classification: Some(Classification::Ranged),
            condition: None,
            handedness: None,
        };
        let strength = 2;
        let dexterity = 3;
        let proficiency_bonus = 3;

        let expected_attack = Some(ConditionalRoll::new_unsafe(1, 20, 2, None));
        let expected_damage = Some(Roll::new_unsafe(1, 6, 2));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            false,
            false,
        );
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false, None);

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_weapon_roll_without_thrown() {
        let roll = WeaponAttackRoll {
            weapon: WeaponName::Greatsword,
            classification: Some(Classification::Ranged),
            condition: None,
            handedness: None,
        };
        let strength = 2;
        let dexterity = 3;
        let proficiency_bonus = 3;

        let expected_attack = Some(ConditionalRoll::new_unsafe(1, 20, 3, None));
        let expected_damage = Some(Roll::new_unsafe(1, 4, 3));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            false,
            false,
        );
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false, None);

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_weapon_roll_with_martial_arts_both_higher() {
        let roll = WeaponAttackRoll {
            weapon: WeaponName::Shortsword,
            classification: None,
            condition: None,
            handedness: None,
        };
        let strength = 2;
        let dexterity = 3;
        let proficiency_bonus = 3;

        let expected_attack = Some(ConditionalRoll::new_unsafe(1, 20, 6, None));
        let expected_damage = Some(Roll::new_unsafe(1, 8, 3));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            true,
            true,
        );
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false, Some(8));

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_weapon_roll_with_martial_arts_dexterity_lower() {
        let roll = WeaponAttackRoll {
            weapon: WeaponName::Shortsword,
            classification: None,
            condition: None,
            handedness: None,
        };
        let strength = 2;
        let dexterity = 1;
        let proficiency_bonus = 3;

        let expected_attack = Some(ConditionalRoll::new_unsafe(1, 20, 5, None));
        let expected_damage = Some(Roll::new_unsafe(1, 8, 2));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            true,
            true,
        );
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false, Some(8));

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_weapon_roll_with_martial_arts_damage_die_lower() {
        let roll = WeaponAttackRoll {
            weapon: WeaponName::Shortsword,
            classification: None,
            condition: None,
            handedness: None,
        };
        let strength = 2;
        let dexterity = 3;
        let proficiency_bonus = 3;

        let expected_attack = Some(ConditionalRoll::new_unsafe(1, 20, 6, None));
        let expected_damage = Some(Roll::new_unsafe(1, 6, 3));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            true,
            true,
        );
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false, Some(4));

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_weapon_roll_with_martial_arts_both_lower() {
        let roll = WeaponAttackRoll {
            weapon: WeaponName::Shortsword,
            classification: None,
            condition: None,
            handedness: None,
        };
        let strength = 2;
        let dexterity = 1;
        let proficiency_bonus = 3;

        let expected_attack = Some(ConditionalRoll::new_unsafe(1, 20, 5, None));
        let expected_damage = Some(Roll::new_unsafe(1, 6, 2));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            true,
            true,
        );
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false, Some(4));

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_weapon_roll_with_martial_arts_without_monk_weapon() {
        let roll = WeaponAttackRoll {
            weapon: WeaponName::Maul,
            classification: None,
            condition: None,
            handedness: None,
        };
        let strength = 2;
        let dexterity = 3;
        let proficiency_bonus = 3;

        let expected_attack = Some(ConditionalRoll::new_unsafe(1, 20, 5, None));
        let expected_damage = Some(Roll::new_unsafe(2, 6, 2));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            true,
            true,
        );
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false, Some(8));

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_weapon_roll_connie_one_handed() {
        let roll = WeaponAttackRoll {
            weapon: WeaponName::Longsword,
            classification: None,
            condition: None,
            handedness: Some(Handedness::OneHanded),
        };
        let strength = 2;
        let dexterity = -1;
        let proficiency_bonus = 2;

        let expected_attack = Some(ConditionalRoll::new_unsafe(1, 20, 4, None));
        let expected_damage = Some(Roll::new_unsafe(1, 8, 2));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            true,
            false,
        );
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false, None);

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_weapon_roll_connie_two_handed() {
        let roll = WeaponAttackRoll {
            weapon: WeaponName::Longsword,
            classification: None,
            condition: None,
            handedness: Some(Handedness::TwoHanded),
        };
        let strength = 2;
        let dexterity = -1;
        let proficiency_bonus = 2;

        let expected_attack = Some(ConditionalRoll::new_unsafe(1, 20, 4, None));
        let expected_damage = Some(Roll::new_unsafe(1, 10, 2));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            true,
            false,
        );
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false, None);

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_weapon_roll_loeguo() {
        let roll = WeaponAttackRoll {
            weapon: WeaponName::Shortsword,
            classification: None,
            condition: None,
            handedness: Some(Handedness::OneHanded),
        };
        let strength = 3;
        let dexterity = -1;
        let proficiency_bonus = 2;

        let expected_attack = Some(ConditionalRoll::new_unsafe(1, 20, 5, None));
        let expected_damage = Some(Roll::new_unsafe(1, 6, 3));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            true,
            true,
        );
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false, Some(4));

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_weapon_roll_tocha_longbow() {
        let roll = WeaponAttackRoll {
            weapon: WeaponName::Longbow,
            classification: None,
            condition: None,
            handedness: None,
        };
        let strength = 2;
        let dexterity = 3;
        let proficiency_bonus = 2;

        let expected_attack = Some(ConditionalRoll::new_unsafe(1, 20, 5, None));
        let expected_damage = Some(Roll::new_unsafe(1, 8, 3));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            true,
            false,
        );
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false, None);

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_weapon_roll_tocha_spear_one_handed() {
        let roll = WeaponAttackRoll {
            weapon: WeaponName::Spear,
            classification: None,
            condition: None,
            handedness: Some(Handedness::OneHanded),
        };
        let strength = 2;
        let dexterity = 3;
        let proficiency_bonus = 2;

        let expected_attack = Some(ConditionalRoll::new_unsafe(1, 20, 4, None));
        let expected_damage = Some(Roll::new_unsafe(1, 6, 2));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            true,
            false,
        );
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false, None);

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_weapon_roll_tocha_spear_two_handed() {
        let roll = WeaponAttackRoll {
            weapon: WeaponName::Spear,
            classification: None,
            condition: None,
            handedness: Some(Handedness::TwoHanded),
        };
        let strength = 2;
        let dexterity = 3;
        let proficiency_bonus = 2;

        let expected_attack = Some(ConditionalRoll::new_unsafe(1, 20, 4, None));
        let expected_damage = Some(Roll::new_unsafe(1, 8, 2));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            true,
            false,
        );
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false, None);

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }
}
