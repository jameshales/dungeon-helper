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
    ) -> Option<ConditionalRoll> {
        match self {
            AttackRoll::ImprovisedWeapon(roll) => roll.to_attack_roll(strength, dexterity),
            AttackRoll::UnarmedStrike(roll) => roll.to_attack_roll(strength, proficiency_bonus),
            AttackRoll::Weapon(roll) => {
                roll.to_attack_roll(strength, dexterity, proficiency_bonus, proficiency)
            }
        }
    }

    pub fn to_damage_roll(
        &self,
        strength: Option<i32>,
        dexterity: Option<i32>,
        proficiency_bonus: Option<i32>,
        proficiency: bool,
        critical_hit: bool,
    ) -> Option<ConditionalRoll> {
        match self {
            AttackRoll::ImprovisedWeapon(roll) => {
                roll.to_damage_roll(strength, dexterity, critical_hit)
            }
            AttackRoll::UnarmedStrike(roll) => {
                roll.to_damage_roll(strength, proficiency_bonus, critical_hit)
            }
            AttackRoll::Weapon(roll) => roll.to_damage_roll(
                strength,
                dexterity,
                proficiency_bonus,
                proficiency,
                critical_hit,
            ),
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
    ) -> Option<ConditionalRoll> {
        let multiplier = critical_hit_multiplier(critical_hit);
        let modifier = self.modifier(strength, dexterity)?;
        Some(ConditionalRoll::new_unsafe(multiplier, 4, modifier, None))
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
        proficiency_bonus: Option<i32>,
    ) -> Option<ConditionalRoll> {
        Some(ConditionalRoll::new_unsafe(
            1,
            20,
            strength? + proficiency_bonus?,
            self.condition,
        ))
    }

    pub fn to_damage_roll(
        &self,
        strength: Option<i32>,
        proficiency_bonus: Option<i32>,
        critical_hit: bool,
    ) -> Option<ConditionalRoll> {
        let multiplier = critical_hit_multiplier(critical_hit);
        Some(ConditionalRoll::new_unsafe(
            multiplier,
            4,
            strength? + proficiency_bonus?,
            None,
        ))
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
    ) -> Option<ConditionalRoll> {
        let modifier = self.modifier(strength, dexterity, proficiency_bonus, proficiency);
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
        proficiency_bonus: Option<i32>,
        proficiency: bool,
        critical_hit: bool,
    ) -> Option<ConditionalRoll> {
        let weapon = self.weapon.to_weapon();
        let used_with_correct_classification = self.classification.iter().all(|c| {
            *c == weapon.classification || (*c == Classification::Ranged && weapon.thrown)
        });
        let roll = if used_with_correct_classification {
            weapon
                .versatile
                .filter(|_| self.handedness == Some(Handedness::TwoHanded))
                .unwrap_or(weapon.damage)
        } else {
            Roll::new_unsafe(1, 4, 0)
        };

        let multiplier = critical_hit_multiplier(critical_hit);
        let modifier = self.modifier(strength, dexterity, proficiency_bonus, proficiency);
        Some(ConditionalRoll::from_roll(
            roll.multiply_rolls(multiplier).add_modifier(modifier?),
            None,
        ))
    }

    fn modifier(
        &self,
        strength: Option<i32>,
        dexterity: Option<i32>,
        proficiency_bonus: Option<i32>,
        proficiency: bool,
    ) -> Option<i32> {
        let weapon = self.weapon.to_weapon();
        let proficiency_bonus =
            proficiency_bonus
                .map(|proficiency_bonus| if proficiency { proficiency_bonus } else { 0 });
        let modifier = match (self.classification.unwrap_or(weapon.classification), weapon.classification, weapon.thrown, weapon.finesse) {
            // Use a melee weapon as a melee weapon
            (Classification::Melee, Classification::Melee, _, false)
            // Use a thrown melee weapon as a ranged weapon
            | (Classification::Ranged, Classification::Melee, true, false) => strength? + proficiency_bonus?,
            // Use a ranged weapon as a ranged weapon
            (Classification::Ranged, Classification::Ranged, _, _) => dexterity? + proficiency_bonus?,
            // Use a melee weapon with finesse as a melee weapon
            (Classification::Melee, Classification::Melee, _, true)
            // Use a thrown melee weapon with finesse as a ranged weapon
            | (Classification::Ranged, Classification::Melee, true, true) => max(strength?, dexterity?) + proficiency_bonus?,
            // Use a ranged weapon as a melee weapon (counts as improvised)
            (Classification::Melee, Classification::Ranged, _, _) => strength?,
            // Use a melee weapon as a ranged weapon (counts as improvised)
            (Classification::Ranged, Classification::Melee, false, _) => dexterity?,
        };
        Some(modifier)
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
        let expected_damage = Some(ConditionalRoll::new_unsafe(1, 4, 2, None));

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

        let expected_damage = Some(ConditionalRoll::new_unsafe(2, 4, 2, None));

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
        let expected_damage = Some(ConditionalRoll::new_unsafe(1, 4, 2, None));

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
        let expected_damage = Some(ConditionalRoll::new_unsafe(1, 4, 2, None));

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
        let expected_damage = Some(ConditionalRoll::new_unsafe(1, 4, 3, None));

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
        let expected_damage = Some(ConditionalRoll::new_unsafe(1, 4, 3, None));

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
        let expected_damage = Some(ConditionalRoll::new_unsafe(1, 4, 3, None));

        let actual_attack = roll.to_attack_roll(Some(strength), Some(dexterity));
        let actual_damage = roll.to_damage_roll(Some(strength), Some(dexterity), false);

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_unarmed_strike_roll() {
        let roll = UnarmedStrikeAttackRoll { condition: None };
        let strength = 2;
        let proficiency_bonus = 3;

        let expected_attack = Some(ConditionalRoll::new_unsafe(1, 20, 5, None));
        let expected_damage = Some(ConditionalRoll::new_unsafe(1, 4, 5, None));

        let actual_attack = roll.to_attack_roll(Some(strength), Some(proficiency_bonus));
        let actual_damage = roll.to_damage_roll(Some(strength), Some(proficiency_bonus), false);

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_unarmed_strike_roll_with_critical_hit() {
        let roll = UnarmedStrikeAttackRoll { condition: None };
        let strength = 2;
        let proficiency_bonus = 3;

        let expected_damage = Some(ConditionalRoll::new_unsafe(2, 4, 5, None));

        let actual_damage = roll.to_damage_roll(Some(strength), Some(proficiency_bonus), true);

        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_unarmed_strike_roll_with_advantage() {
        let roll = UnarmedStrikeAttackRoll {
            condition: Some(Condition::Advantage),
        };
        let strength = -1;
        let proficiency_bonus = 3;

        let expected_attack = Some(ConditionalRoll::new_unsafe(
            1,
            20,
            2,
            Some(Condition::Advantage),
        ));
        let expected_damage = Some(ConditionalRoll::new_unsafe(1, 4, 2, None));

        let actual_attack = roll.to_attack_roll(Some(strength), Some(proficiency_bonus));
        let actual_damage = roll.to_damage_roll(Some(strength), Some(proficiency_bonus), false);

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_unarmed_strike_roll_with_disadvantage() {
        let roll = UnarmedStrikeAttackRoll {
            condition: Some(Condition::Disadvantage),
        };
        let strength = 2;
        let proficiency_bonus = 1;

        let expected_attack = Some(ConditionalRoll::new_unsafe(
            1,
            20,
            3,
            Some(Condition::Disadvantage),
        ));
        let expected_damage = Some(ConditionalRoll::new_unsafe(1, 4, 3, None));

        let actual_attack = roll.to_attack_roll(Some(strength), Some(proficiency_bonus));
        let actual_damage = roll.to_damage_roll(Some(strength), Some(proficiency_bonus), false);

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_unarmed_strike_roll_without_strength() {
        let roll = UnarmedStrikeAttackRoll { condition: None };
        let proficiency_bonus = 3;

        let expected_attack = None;
        let expected_damage = None;

        let actual_attack = roll.to_attack_roll(None, Some(proficiency_bonus));
        let actual_damage = roll.to_damage_roll(None, Some(proficiency_bonus), false);

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }

    #[test]
    fn test_unarmed_strike_roll_without_proficiency_bonus() {
        let roll = UnarmedStrikeAttackRoll { condition: None };
        let strength = 2;

        let expected_attack = None;
        let expected_damage = None;

        let actual_attack = roll.to_attack_roll(Some(strength), None);
        let actual_damage = roll.to_damage_roll(Some(strength), None, false);

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
        let expected_damage = Some(ConditionalRoll::new_unsafe(2, 6, 2, None));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            false,
        );
        let actual_damage = roll.to_damage_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            false,
            false,
        );

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
        let proficiency_bonus = 3;

        let expected_damage = Some(ConditionalRoll::new_unsafe(4, 6, 2, None));

        let actual_damage = roll.to_damage_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            false,
            true,
        );

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
        let expected_damage = Some(ConditionalRoll::new_unsafe(2, 6, 2, None));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            false,
        );
        let actual_damage = roll.to_damage_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            false,
            false,
        );

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
        let expected_damage = Some(ConditionalRoll::new_unsafe(2, 6, 2, None));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            false,
        );
        let actual_damage = roll.to_damage_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            false,
            false,
        );

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
        let expected_damage = Some(ConditionalRoll::new_unsafe(1, 8, 3, None));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            false,
        );
        let actual_damage = roll.to_damage_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            false,
            false,
        );

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
        let expected_damage = Some(ConditionalRoll::new_unsafe(1, 6, 2, None));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            false,
        );
        let actual_damage = roll.to_damage_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            false,
            false,
        );

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
        let expected_damage = Some(ConditionalRoll::new_unsafe(1, 4, 3, None));

        let actual_attack = roll.to_attack_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            false,
        );
        let actual_damage = roll.to_damage_roll(
            Some(strength),
            Some(dexterity),
            Some(proficiency_bonus),
            false,
            false,
        );

        assert_eq!(actual_attack, expected_attack);
        assert_eq!(actual_damage, expected_damage);
    }
}

fn critical_hit_multiplier(critical_hit: bool) -> usize {
    if critical_hit {
        2
    } else {
        1
    }
}
