use crate::roll::Roll;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ValueRef};
use std::error;
use std::fmt;

pub struct Weapon {
    pub name: WeaponName,
    pub category: Category,
    pub classification: Classification,
    pub damage: Roll,
    pub damage_type: DamageType,
    pub two_handed: bool,
    pub thrown: bool,
    pub finesse: bool,
    pub versatile: Option<Roll>,
    pub heavy: bool,
}

impl Weapon {
    pub fn is_monk_weapon(&self) -> bool {
        self.name == WeaponName::Shortsword
            || (self.category == Category::Simple
                && self.classification == Classification::Melee
                && !self.two_handed
                && !self.heavy)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WeaponName {
    Battleaxe,
    Club,
    CrossbowHand,
    CrossbowHeavy,
    CrossbowLight,
    Dagger,
    Dart,
    Flail,
    Glaive,
    Greataxe,
    Greatclub,
    Greatsword,
    Halberd,
    Handaxe,
    Javelin,
    Lance,
    LightHammer,
    Longbow,
    Longsword,
    Mace,
    Maul,
    Morningstar,
    Pike,
    Quarterstaff,
    Rapier,
    Scimitar,
    Shortbow,
    Shortsword,
    Sickle,
    Sling,
    Spear,
    Trident,
    WarPick,
    Warhammer,
    Whip,
}

impl WeaponName {
    pub fn as_str(&self) -> &str {
        match self {
            WeaponName::Battleaxe => "Battleaxe",
            WeaponName::Club => "Club",
            WeaponName::CrossbowHand => "Hand Crossbow",
            WeaponName::CrossbowHeavy => "Heavy Crossbow",
            WeaponName::CrossbowLight => "Light Crossbow",
            WeaponName::Dagger => "Dagger",
            WeaponName::Dart => "Dart",
            WeaponName::Flail => "Flail",
            WeaponName::Glaive => "Glaive",
            WeaponName::Greataxe => "Greataxe",
            WeaponName::Greatclub => "Greatclub",
            WeaponName::Greatsword => "Greatsword",
            WeaponName::Halberd => "Halberd",
            WeaponName::Handaxe => "Handaxe",
            WeaponName::Javelin => "Javelin",
            WeaponName::Lance => "Lance",
            WeaponName::LightHammer => "Light Hammer",
            WeaponName::Longbow => "Longbow",
            WeaponName::Longsword => "Longsword",
            WeaponName::Mace => "Mace",
            WeaponName::Maul => "Maul",
            WeaponName::Morningstar => "Morningstar",
            WeaponName::Pike => "Pike",
            WeaponName::Quarterstaff => "Quarterstaff",
            WeaponName::Rapier => "Rapier",
            WeaponName::Scimitar => "Scimitar",
            WeaponName::Shortbow => "Shortbow",
            WeaponName::Shortsword => "Shortsword",
            WeaponName::Sickle => "Sickle",
            WeaponName::Sling => "Sling",
            WeaponName::Spear => "Spear",
            WeaponName::Trident => "Trident",
            WeaponName::WarPick => "War Pick",
            WeaponName::Warhammer => "Warhammer",
            WeaponName::Whip => "Whip",
        }
    }

    pub fn parse(name: &str) -> Option<WeaponName> {
        match name.to_lowercase().as_ref() {
            "battleaxe" => Some(WeaponName::Battleaxe),
            "club" => Some(WeaponName::Club),
            "hand crossbow" => Some(WeaponName::CrossbowHand),
            "heavy crossbow" => Some(WeaponName::CrossbowHeavy),
            "light crossbow" => Some(WeaponName::CrossbowLight),
            "dagger" => Some(WeaponName::Dagger),
            "dart" => Some(WeaponName::Dart),
            "flail" => Some(WeaponName::Flail),
            "glaive" => Some(WeaponName::Glaive),
            "greataxe" => Some(WeaponName::Greataxe),
            "greatclub" => Some(WeaponName::Greatclub),
            "greatsword" => Some(WeaponName::Greatsword),
            "halberd" => Some(WeaponName::Halberd),
            "handaxe" => Some(WeaponName::Handaxe),
            "javelin" => Some(WeaponName::Javelin),
            "lance" => Some(WeaponName::Lance),
            "light hammer" => Some(WeaponName::LightHammer),
            "longbow" => Some(WeaponName::Longbow),
            "longsword" => Some(WeaponName::Longsword),
            "mace" => Some(WeaponName::Mace),
            "maul" => Some(WeaponName::Maul),
            "morningstar" => Some(WeaponName::Morningstar),
            "pike" => Some(WeaponName::Pike),
            "quarterstaff" => Some(WeaponName::Quarterstaff),
            "rapier" => Some(WeaponName::Rapier),
            "scimitar" => Some(WeaponName::Scimitar),
            "shortbow" => Some(WeaponName::Shortbow),
            "shortsword" => Some(WeaponName::Shortsword),
            "sickle" => Some(WeaponName::Sickle),
            "sling" => Some(WeaponName::Sling),
            "spear" => Some(WeaponName::Spear),
            "trident" => Some(WeaponName::Trident),
            "war pick" => Some(WeaponName::WarPick),
            "warhammer" => Some(WeaponName::Warhammer),
            "whip" => Some(WeaponName::Whip),
            _ => None,
        }
    }

    pub fn to_weapon(&self) -> &Weapon {
        match self {
            WeaponName::Battleaxe => &BATTLEAXE,
            WeaponName::Club => &CLUB,
            WeaponName::CrossbowHand => &CROSSBOW_HAND,
            WeaponName::CrossbowHeavy => &CROSSBOW_HEAVY,
            WeaponName::CrossbowLight => &CROSSBOW_LIGHT,
            WeaponName::Dagger => &DAGGER,
            WeaponName::Dart => &DART,
            WeaponName::Flail => &FLAIL,
            WeaponName::Glaive => &GLAIVE,
            WeaponName::Greataxe => &GREATAXE,
            WeaponName::Greatclub => &GREATCLUB,
            WeaponName::Greatsword => &GREATSWORD,
            WeaponName::Halberd => &HALBERD,
            WeaponName::Handaxe => &HANDAXE,
            WeaponName::Javelin => &JAVELIN,
            WeaponName::Lance => &LANCE,
            WeaponName::LightHammer => &LIGHT_HAMMER,
            WeaponName::Longbow => &LONGBOW,
            WeaponName::Longsword => &LONGSWORD,
            WeaponName::Mace => &MACE,
            WeaponName::Maul => &MAUL,
            WeaponName::Morningstar => &MORNINGSTAR,
            WeaponName::Pike => &PIKE,
            WeaponName::Quarterstaff => &QUARTERSTAFF,
            WeaponName::Rapier => &RAPIER,
            WeaponName::Scimitar => &SCIMITAR,
            WeaponName::Shortbow => &SHORTBOW,
            WeaponName::Shortsword => &SHORTSWORD,
            WeaponName::Sickle => &SICKLE,
            WeaponName::Sling => &SLING,
            WeaponName::Spear => &SPEAR,
            WeaponName::Trident => &TRIDENT,
            WeaponName::WarPick => &WAR_PICK,
            WeaponName::Warhammer => &WARHAMMER,
            WeaponName::Whip => &WHIP,
        }
    }
}

impl fmt::Display for WeaponName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl FromSql for WeaponName {
    fn column_result(value: ValueRef) -> FromSqlResult<WeaponName> {
        value.as_str().and_then(|string| {
            WeaponName::parse(string).ok_or_else(|| {
                FromSqlError::Other(Box::new(InvalidWeaponNameValueError {
                    value: string.to_owned(),
                }))
            })
        })
    }
}

#[derive(Debug)]
struct InvalidWeaponNameValueError {
    value: String,
}

impl fmt::Display for InvalidWeaponNameValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid value for weapon name (value = {})", self.value)
    }
}

impl error::Error for InvalidWeaponNameValueError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AmbiguousWeaponName {
    Axe,
    Bow,
    Crossbow,
    Hammer,
    Sword,
}

impl AmbiguousWeaponName {
    pub fn as_str(&self) -> &str {
        match self {
            AmbiguousWeaponName::Axe => "Axe",
            AmbiguousWeaponName::Bow => "Bow",
            AmbiguousWeaponName::Crossbow => "Crossbow",
            AmbiguousWeaponName::Hammer => "Hammer",
            AmbiguousWeaponName::Sword => "Sword",
        }
    }

    pub fn parse(name: &str) -> Option<AmbiguousWeaponName> {
        match name.to_lowercase().as_str() {
            "axe" => Some(AmbiguousWeaponName::Axe),
            "bow" => Some(AmbiguousWeaponName::Bow),
            "crossbow" => Some(AmbiguousWeaponName::Crossbow),
            "hammer" => Some(AmbiguousWeaponName::Hammer),
            "sword" => Some(AmbiguousWeaponName::Sword),
            _ => None,
        }
    }

    pub fn message(&self) -> &str {
        match self {
            AmbiguousWeaponName::Axe => "Try using a more specific weapon name, such as \"Battleaxe\", \"Greataxe\", or \"Handaxe\".",
            AmbiguousWeaponName::Bow => "Try using a more specific weapon name, such as \"Shortbow\", or \"Longbow\".",
            AmbiguousWeaponName::Crossbow => "Try using a more specific weapon name, such as \"Hand Crossbow\", \"Heavy Crossbow\", or \"Light Crossbow\".",
            AmbiguousWeaponName::Hammer => "Try using a more specific weapon name, such as \"Light Hammer\", or \"Warhammer\".",
            AmbiguousWeaponName::Sword => "Try using a more specific weapon name, such as \"Greatsword\", \"Longsword\", or \"Shortsword\".",
        }
    }
}

impl fmt::Display for AmbiguousWeaponName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Category {
    Simple,
    Martial,
}

impl Category {
    pub fn parse(string: &str) -> Option<Category> {
        match string.to_lowercase().as_ref() {
            "simple" => Some(Category::Simple),
            "martial" => Some(Category::Martial),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Category::Simple => "Simple",
            Category::Martial => "Martial",
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl FromSql for Category {
    fn column_result(value: ValueRef) -> FromSqlResult<Category> {
        value.as_str().and_then(|string| {
            Category::parse(string).ok_or_else(|| {
                FromSqlError::Other(Box::new(InvalidCategoryValueError {
                    value: string.to_owned(),
                }))
            })
        })
    }
}

#[derive(Debug)]
struct InvalidCategoryValueError {
    value: String,
}

impl fmt::Display for InvalidCategoryValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Invalid value for weapon category (value = {})",
            self.value
        )
    }
}

impl error::Error for InvalidCategoryValueError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Classification {
    Melee,
    Ranged,
}

impl Classification {
    pub fn parse(string: &str) -> Option<Classification> {
        match string.to_lowercase().as_ref() {
            "melee" => Some(Classification::Melee),
            "ranged" => Some(Classification::Ranged),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Classification::Melee => "Melee",
            Classification::Ranged => "Ranged",
        }
    }
}

impl fmt::Display for Classification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum DamageType {
    Bludgeoning,
    Piercing,
    Slashing,
}

static BATTLEAXE: Weapon = Weapon {
    name: WeaponName::Battleaxe,
    category: Category::Martial,
    classification: Classification::Melee,
    damage: Roll::new_unsafe(1, 8, 0),
    damage_type: DamageType::Slashing,
    two_handed: false,
    thrown: false,
    finesse: false,
    versatile: Some(Roll::new_unsafe(1, 10, 0)),
    heavy: false,
};

static CLUB: Weapon = Weapon {
    name: WeaponName::Club,
    category: Category::Simple,
    classification: Classification::Melee,
    damage: Roll::new_unsafe(1, 4, 0),
    damage_type: DamageType::Bludgeoning,
    two_handed: false,
    thrown: false,
    finesse: false,
    versatile: None,
    heavy: false,
};

static CROSSBOW_HAND: Weapon = Weapon {
    name: WeaponName::CrossbowHand,
    category: Category::Martial,
    classification: Classification::Ranged,
    damage: Roll::new_unsafe(1, 6, 0),
    damage_type: DamageType::Piercing,
    two_handed: false,
    thrown: false,
    finesse: false,
    versatile: None,
    heavy: false,
};

static CROSSBOW_HEAVY: Weapon = Weapon {
    name: WeaponName::CrossbowHeavy,
    category: Category::Martial,
    classification: Classification::Melee,
    damage: Roll::new_unsafe(1, 10, 0),
    damage_type: DamageType::Piercing,
    two_handed: true,
    thrown: false,
    finesse: false,
    versatile: None,
    heavy: true,
};

static CROSSBOW_LIGHT: Weapon = Weapon {
    name: WeaponName::CrossbowLight,
    category: Category::Simple,
    classification: Classification::Ranged,
    damage: Roll::new_unsafe(1, 8, 0),
    damage_type: DamageType::Piercing,
    two_handed: true,
    thrown: false,
    finesse: false,
    versatile: None,
    heavy: false,
};

static DAGGER: Weapon = Weapon {
    name: WeaponName::Dagger,
    category: Category::Simple,
    classification: Classification::Melee,
    damage: Roll::new_unsafe(1, 4, 0),
    damage_type: DamageType::Piercing,
    two_handed: false,
    thrown: true,
    finesse: true,
    versatile: None,
    heavy: false,
};

static DART: Weapon = Weapon {
    name: WeaponName::Dart,
    category: Category::Simple,
    classification: Classification::Ranged,
    damage: Roll::new_unsafe(1, 4, 0),
    damage_type: DamageType::Piercing,
    two_handed: false,
    thrown: true,
    finesse: true,
    versatile: None,
    heavy: false,
};

static FLAIL: Weapon = Weapon {
    name: WeaponName::Flail,
    category: Category::Martial,
    classification: Classification::Melee,
    damage: Roll::new_unsafe(1, 8, 0),
    damage_type: DamageType::Bludgeoning,
    two_handed: false,
    thrown: false,
    finesse: false,
    versatile: None,
    heavy: false,
};

static GLAIVE: Weapon = Weapon {
    name: WeaponName::Glaive,
    category: Category::Martial,
    classification: Classification::Melee,
    damage: Roll::new_unsafe(1, 10, 0),
    damage_type: DamageType::Slashing,
    two_handed: true,
    thrown: false,
    finesse: false,
    versatile: None,
    heavy: true,
};

static GREATAXE: Weapon = Weapon {
    name: WeaponName::Greataxe,
    category: Category::Martial,
    classification: Classification::Melee,
    damage: Roll::new_unsafe(1, 12, 0),
    damage_type: DamageType::Slashing,
    two_handed: true,
    thrown: false,
    finesse: false,
    versatile: None,
    heavy: true,
};

static GREATCLUB: Weapon = Weapon {
    name: WeaponName::Greatclub,
    category: Category::Simple,
    classification: Classification::Melee,
    damage: Roll::new_unsafe(1, 8, 0),
    damage_type: DamageType::Bludgeoning,
    two_handed: true,
    thrown: false,
    finesse: false,
    versatile: None,
    heavy: false,
};

static GREATSWORD: Weapon = Weapon {
    name: WeaponName::Greatsword,
    category: Category::Martial,
    classification: Classification::Melee,
    damage: Roll::new_unsafe(2, 6, 0),
    damage_type: DamageType::Slashing,
    two_handed: true,
    thrown: false,
    finesse: false,
    versatile: None,
    heavy: true,
};

static HALBERD: Weapon = Weapon {
    name: WeaponName::Halberd,
    category: Category::Martial,
    classification: Classification::Melee,
    damage: Roll::new_unsafe(1, 10, 0),
    damage_type: DamageType::Slashing,
    two_handed: true,
    thrown: false,
    finesse: false,
    versatile: None,
    heavy: true,
};

static HANDAXE: Weapon = Weapon {
    name: WeaponName::Handaxe,
    category: Category::Simple,
    classification: Classification::Melee,
    damage: Roll::new_unsafe(1, 6, 0),
    damage_type: DamageType::Slashing,
    two_handed: false,
    thrown: true,
    finesse: false,
    versatile: None,
    heavy: false,
};

static JAVELIN: Weapon = Weapon {
    name: WeaponName::Javelin,
    category: Category::Simple,
    classification: Classification::Melee,
    damage: Roll::new_unsafe(1, 6, 0),
    damage_type: DamageType::Piercing,
    two_handed: false,
    thrown: true,
    finesse: false,
    versatile: None,
    heavy: false,
};

static LANCE: Weapon = Weapon {
    name: WeaponName::Lance,
    category: Category::Martial,
    classification: Classification::Melee,
    damage: Roll::new_unsafe(1, 12, 0),
    damage_type: DamageType::Piercing,
    two_handed: false,
    thrown: false,
    finesse: false,
    versatile: None,
    heavy: false,
};

static LIGHT_HAMMER: Weapon = Weapon {
    name: WeaponName::LightHammer,
    category: Category::Simple,
    classification: Classification::Melee,
    damage: Roll::new_unsafe(1, 4, 0),
    damage_type: DamageType::Bludgeoning,
    two_handed: false,
    thrown: true,
    finesse: false,
    versatile: None,
    heavy: false,
};

static LONGBOW: Weapon = Weapon {
    name: WeaponName::Longbow,
    category: Category::Martial,
    classification: Classification::Ranged,
    damage: Roll::new_unsafe(1, 8, 0),
    damage_type: DamageType::Piercing,
    two_handed: true,
    thrown: false,
    finesse: false,
    versatile: None,
    heavy: true,
};

static LONGSWORD: Weapon = Weapon {
    name: WeaponName::Longsword,
    category: Category::Martial,
    classification: Classification::Melee,
    damage: Roll::new_unsafe(1, 8, 0),
    damage_type: DamageType::Slashing,
    two_handed: true,
    thrown: false,
    finesse: false,
    versatile: Some(Roll::new_unsafe(1, 10, 0)),
    heavy: false,
};

static MACE: Weapon = Weapon {
    name: WeaponName::Mace,
    category: Category::Simple,
    classification: Classification::Melee,
    damage: Roll::new_unsafe(1, 6, 0),
    damage_type: DamageType::Bludgeoning,
    two_handed: false,
    thrown: false,
    finesse: false,
    versatile: None,
    heavy: false,
};

static MAUL: Weapon = Weapon {
    name: WeaponName::Maul,
    category: Category::Martial,
    classification: Classification::Melee,
    damage: Roll::new_unsafe(2, 6, 0),
    damage_type: DamageType::Bludgeoning,
    two_handed: true,
    thrown: false,
    finesse: false,
    versatile: None,
    heavy: true,
};

static MORNINGSTAR: Weapon = Weapon {
    name: WeaponName::Morningstar,
    category: Category::Martial,
    classification: Classification::Melee,
    damage: Roll::new_unsafe(1, 8, 0),
    damage_type: DamageType::Piercing,
    two_handed: false,
    thrown: false,
    finesse: false,
    versatile: None,
    heavy: false,
};

static PIKE: Weapon = Weapon {
    name: WeaponName::Pike,
    category: Category::Martial,
    classification: Classification::Melee,
    damage: Roll::new_unsafe(1, 10, 0),
    damage_type: DamageType::Piercing,
    two_handed: true,
    thrown: false,
    finesse: false,
    versatile: None,
    heavy: true,
};

static QUARTERSTAFF: Weapon = Weapon {
    name: WeaponName::Quarterstaff,
    category: Category::Simple,
    classification: Classification::Melee,
    damage: Roll::new_unsafe(1, 6, 0),
    damage_type: DamageType::Bludgeoning,
    two_handed: false,
    thrown: false,
    finesse: false,
    versatile: Some(Roll::new_unsafe(1, 8, 0)),
    heavy: false,
};

static RAPIER: Weapon = Weapon {
    name: WeaponName::Rapier,
    category: Category::Martial,
    classification: Classification::Melee,
    damage: Roll::new_unsafe(1, 8, 0),
    damage_type: DamageType::Piercing,
    two_handed: false,
    thrown: false,
    finesse: true,
    versatile: None,
    heavy: false,
};

static SCIMITAR: Weapon = Weapon {
    name: WeaponName::Scimitar,
    category: Category::Martial,
    classification: Classification::Melee,
    damage: Roll::new_unsafe(1, 6, 0),
    damage_type: DamageType::Slashing,
    two_handed: false,
    thrown: false,
    finesse: true,
    versatile: None,
    heavy: false,
};

static SHORTBOW: Weapon = Weapon {
    name: WeaponName::Shortbow,
    category: Category::Simple,
    classification: Classification::Ranged,
    damage: Roll::new_unsafe(1, 6, 0),
    damage_type: DamageType::Piercing,
    two_handed: true,
    thrown: false,
    finesse: false,
    versatile: None,
    heavy: false,
};

static SHORTSWORD: Weapon = Weapon {
    name: WeaponName::Shortsword,
    category: Category::Martial,
    classification: Classification::Melee,
    damage: Roll::new_unsafe(1, 6, 0),
    damage_type: DamageType::Piercing,
    two_handed: false,
    thrown: false,
    finesse: true,
    versatile: None,
    heavy: false,
};

static SICKLE: Weapon = Weapon {
    name: WeaponName::Sickle,
    category: Category::Simple,
    classification: Classification::Melee,
    damage: Roll::new_unsafe(1, 4, 0),
    damage_type: DamageType::Slashing,
    two_handed: false,
    thrown: false,
    finesse: false,
    versatile: None,
    heavy: false,
};

static SLING: Weapon = Weapon {
    name: WeaponName::Sling,
    category: Category::Simple,
    classification: Classification::Ranged,
    damage: Roll::new_unsafe(1, 4, 0),
    damage_type: DamageType::Bludgeoning,
    two_handed: false,
    thrown: false,
    finesse: false,
    versatile: None,
    heavy: false,
};

static SPEAR: Weapon = Weapon {
    name: WeaponName::Spear,
    category: Category::Simple,
    classification: Classification::Melee,
    damage: Roll::new_unsafe(1, 6, 0),
    damage_type: DamageType::Piercing,
    two_handed: false,
    thrown: true,
    finesse: false,
    versatile: Some(Roll::new_unsafe(1, 8, 0)),
    heavy: false,
};

static TRIDENT: Weapon = Weapon {
    name: WeaponName::Trident,
    category: Category::Martial,
    classification: Classification::Melee,
    damage: Roll::new_unsafe(1, 6, 0),
    damage_type: DamageType::Piercing,
    two_handed: false,
    thrown: true,
    finesse: false,
    versatile: Some(Roll::new_unsafe(1, 8, 0)),
    heavy: false,
};

static WAR_PICK: Weapon = Weapon {
    name: WeaponName::WarPick,
    category: Category::Martial,
    classification: Classification::Melee,
    damage: Roll::new_unsafe(1, 8, 0),
    damage_type: DamageType::Piercing,
    two_handed: false,
    thrown: false,
    finesse: false,
    versatile: None,
    heavy: false,
};

static WARHAMMER: Weapon = Weapon {
    name: WeaponName::Warhammer,
    category: Category::Martial,
    classification: Classification::Melee,
    damage: Roll::new_unsafe(1, 8, 0),
    damage_type: DamageType::Bludgeoning,
    two_handed: false,
    thrown: false,
    finesse: false,
    versatile: Some(Roll::new_unsafe(1, 10, 0)),
    heavy: false,
};

static WHIP: Weapon = Weapon {
    name: WeaponName::Whip,
    category: Category::Martial,
    classification: Classification::Melee,
    damage: Roll::new_unsafe(1, 4, 0),
    damage_type: DamageType::Slashing,
    two_handed: false,
    thrown: false,
    finesse: true,
    versatile: None,
    heavy: false,
};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_monk_weapon() {
        // Shortsword - special case
        assert_eq!(SHORTSWORD.is_monk_weapon(), true);

        // One-handed, simple, melee
        assert_eq!(CLUB.is_monk_weapon(), true);
        assert_eq!(DAGGER.is_monk_weapon(), true);
        assert_eq!(QUARTERSTAFF.is_monk_weapon(), true);

        // Martial
        assert_eq!(FLAIL.is_monk_weapon(), false);
        assert_eq!(GREATCLUB.is_monk_weapon(), false);

        // Ranged
        assert_eq!(CROSSBOW_HEAVY.is_monk_weapon(), false);
        assert_eq!(CROSSBOW_LIGHT.is_monk_weapon(), false);
        assert_eq!(SHORTBOW.is_monk_weapon(), false);
    }
}
