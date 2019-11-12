use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use regex::Regex;
use std::fmt;

/// The maximum number of dice that may be rolled at one time.
pub const MAXIMUM_ROLLS: usize = 100;

/// The maximum number of individual dice rolls that will be displayed in full.
pub const MAXIMUM_ROLLS_DISPLAY: usize = 10;

/// The maximum number of sides a die may have.
pub const MAXIMUM_SIDES: i32 = 100;

/// Determines the conditions under which a roll occurs - advantage, disadvantage, or normal.
///
/// A roll with advantage involves performing the roll twice and taking the highest result, whereas
/// a roll with disadvantage involves performing the roll twice and taking the lowest result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Condition {
    Advantage,
    Normal,
    Disadvantage,
}

/// Represents a dice roll that might occur in Dungeons and Dragons 5th edition.
///
/// A dice roll involves rolling a number of dice, each with a number of sides. The sum of the
/// rolled dice is added to the modifier, which may be positive or negative. The roll may have
/// advantage or disadvantage.
#[derive(Debug, Eq, PartialEq)]
pub struct Roll {
    rolls: usize,
    sides: i32,
    modifier: i32,
    condition: Condition,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Critical {
    Success,
    Failure,
}

#[derive(Debug, Eq, PartialEq)]
pub struct RollResult {
    result: i32,
    dice: Vec<i32>,
    modifier: i32,
    critical: Option<Critical>,
}

impl fmt::Display for RollResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.result).and({
            if self.dice.len() > 1 || self.modifier != 0 {
                let mut iter = self.dice.iter().take(MAXIMUM_ROLLS_DISPLAY);
                iter.next().map_or(Result::Ok(()), |head| {
                    iter.fold(write!(f, " ({}", head), |result, die| {
                        result.and(write!(f, " + {}", die))
                    })
                    .and(if self.dice.len() > MAXIMUM_ROLLS_DISPLAY {
                        write!(f, " + …")
                    } else {
                        Result::Ok(())
                    })
                    .and(if self.modifier > 0 {
                        write!(f, " + __{}__)", self.modifier)
                    } else if self.modifier < 0 {
                        write!(f, " - __{}__)", -self.modifier)
                    } else {
                        write!(f, ")")
                    })
                })
            } else {
                Result::Ok(())
            }
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ConditionalRollResult {
    primary: RollResult,
    secondary: Option<RollResult>,
}

impl fmt::Display for ConditionalRollResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.primary).and(
            self.secondary.as_ref().map_or(Result::Ok(()), |secondary| {
                write!(f, " / ~~{}~~", secondary)
            }),
        )
    }
}

/// Represents an error that might occur when creating a roll.
///
/// A roll must have involve a positive number of rolls of dice with a positive number of sides.
/// The number of rolls and sides must not be more than 100.
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    RollsNonPositive,
    RollsTooGreat,
    SidesNonPositive,
    SidesTooGreat,
}

impl Error {
    /// Returns a human-friendly explanation of the error.
    pub fn message(&self) -> &str {
        match self {
            Error::RollsNonPositive => "Must roll at least one die",
            Error::RollsTooGreat => "Must roll no more than 100 dice.",
            Error::SidesNonPositive => "Dice must have at least one side.",
            Error::SidesTooGreat => "Dice must have no more than 100 sides.",
        }
    }
}

/// Represents an error that might occur when parsing a roll from a String.
#[derive(Debug, Eq, PartialEq)]
pub enum ParserError {
    InvalidSyntax,
    InvalidValue(Error),
}

impl ParserError {
    /// Returns a human-friendly explanation of the error.
    pub fn message(&self) -> &str {
        match self {
            ParserError::InvalidSyntax => "Invalid syntax.",
            ParserError::InvalidValue(e) => e.message(),
        }
    }
}

impl Roll {
    /// Create a roll, validating that the number of dice being rolled, and the number of sides
    /// each die has, are positive and no more than the maximum allowed values.
    pub fn new(
        rolls: usize,
        sides: i32,
        modifier: i32,
        condition: Condition,
    ) -> Result<Roll, Error> {
        if rolls <= 0 {
            Result::Err(Error::RollsNonPositive)
        } else if rolls > MAXIMUM_ROLLS {
            Result::Err(Error::RollsTooGreat)
        } else if sides <= 0 {
            Result::Err(Error::SidesNonPositive)
        } else if sides > MAXIMUM_SIDES {
            Result::Err(Error::SidesTooGreat)
        } else {
            Result::Ok(Roll {
                rolls,
                sides,
                modifier,
                condition,
            })
        }
    }

    /// Parse a roll from a String using conventional Dungeons and Dragons syntax.
    pub fn parse(string: &str) -> Result<Roll, ParserError> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(\d+)d(\d+)(?: ?(\+|-) ?(\d+))?(?: with (advantage|disadvantage))?$")
                    .unwrap();
        }
        Roll::parse_regex(&RE, string)
    }

    fn parse_regex(regex: &Regex, string: &str) -> Result<Roll, ParserError> {
        regex
            .captures(string)
            .and_then(|captures| {
                let rolls = captures
                    .get(1)
                    .and_then(|m| m.as_str().parse::<usize>().ok());
                let sides = captures.get(2).and_then(|m| m.as_str().parse::<i32>().ok());
                let negative_modifier = captures
                    .get(3)
                    .map(|m| m.as_str())
                    .map_or(false, |v| v == "-");
                let modifier = captures
                    .get(4)
                    .and_then(|m| m.as_str().parse::<i32>().ok())
                    .map(|modifier| {
                        if negative_modifier {
                            -modifier
                        } else {
                            modifier
                        }
                    })
                    .unwrap_or(0);
                let condition = match captures.get(5).map(|m| m.as_str()) {
                    Some("advantage") => Condition::Advantage,
                    Some("disadvantage") => Condition::Disadvantage,
                    _ => Condition::Normal,
                };

                rolls.and_then(|rolls| sides.map(|sides| (rolls, sides, modifier, condition)))
            })
            .ok_or(ParserError::InvalidSyntax)
            .and_then(|(rolls, sides, modifier, condition)| {
                Roll::new(rolls, sides, modifier, condition)
                    .map_err(|e| ParserError::InvalidValue(e))
            })
    }

    /// Roll the dice described by this roll, with any modifier
    pub fn roll<R: Rng>(&self, rng: &mut R) -> ConditionalRollResult {
        let first = self.roll_once(rng);
        let second = self.roll_once(rng);
        match self.condition {
            Condition::Advantage => {
                let (primary, secondary) = if first.result > second.result {
                    (first, second)
                } else {
                    (second, first)
                };
                ConditionalRollResult {
                    primary: primary,
                    secondary: Some(secondary),
                }
            }
            Condition::Normal => ConditionalRollResult {
                primary: first,
                secondary: None,
            },
            Condition::Disadvantage => {
                let (primary, secondary) = if first.result < second.result {
                    (first, second)
                } else {
                    (second, first)
                };
                ConditionalRollResult {
                    primary: primary,
                    secondary: Some(secondary),
                }
            }
        }
    }

    /// Roll the dice once, not taking into acccount advantage or disadvantage. This is repeated in
    /// order to perform a roll with advanatge or disadvantage.
    pub fn roll_once<R: Rng>(&self, rng: &mut R) -> RollResult {
        let dice = self.roll_once_component(rng);
        let sum: i32 = dice.iter().sum();
        let result = sum + self.modifier;
        let critical = if self.rolls == 1 && self.sides == 20 {
            if result == 1 {
                Some(Critical::Failure)
            } else if result == 20 {
                Some(Critical::Success)
            } else {
                None
            }
        } else {
            None
        };
        RollResult {
            result,
            dice,
            modifier: self.modifier,
            critical,
        }
    }

    fn roll_once_component<R: Rng>(&self, rng: &mut R) -> Vec<i32> {
        Uniform::new_inclusive(1, self.sides)
            .sample_iter(rng)
            .take(self.rolls)
            .collect()
    }
}

impl fmt::Display for Roll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}d{}", self.rolls, self.sides)
            .and(if self.modifier > 0 {
                write!(f, " + {}", self.modifier)
            } else if self.modifier < 0 {
                write!(f, " - {}", self.modifier.abs())
            } else {
                Result::Ok(())
            })
            .and(match self.condition {
                Condition::Advantage => write!(f, " with advantage"),
                Condition::Normal => Result::Ok(()),
                Condition::Disadvantage => write!(f, " with disadvantage"),
            })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_simple() {
        let roll = Roll::new(1, 20, 0, Condition::Normal).unwrap();

        let expected = "1d20";
        let actual = format!("{}", roll);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_display_with_positive_modifier() {
        let roll = Roll::new(1, 20, 3, Condition::Normal).unwrap();

        let expected = "1d20 + 3";
        let actual = format!("{}", roll);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_display_with_negative_modifier() {
        let roll = Roll::new(1, 20, -3, Condition::Normal).unwrap();

        let expected = "1d20 - 3";
        let actual = format!("{}", roll);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_display_with_advantage() {
        let roll = Roll::new(1, 20, 0, Condition::Advantage).unwrap();

        let expected = "1d20 with advantage";
        let actual = format!("{}", roll);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_display_with_disadvantage() {
        let roll = Roll::new(1, 20, 0, Condition::Disadvantage).unwrap();

        let expected = "1d20 with disadvantage";
        let actual = format!("{}", roll);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_display_with_modifier_and_advantage() {
        let roll = Roll::new(1, 20, 3, Condition::Advantage).unwrap();

        let expected = "1d20 + 3 with advantage";
        let actual = format!("{}", roll);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_simple() {
        let expected = Result::Ok(Roll::new(1, 20, 0, Condition::Normal).unwrap());
        let actual = Roll::parse("1d20");

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_with_positive_modifier() {
        let expected = Result::Ok(Roll::new(1, 20, 3, Condition::Normal).unwrap());
        let actual = Roll::parse("1d20 + 3");

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_with_negative_modifier() {
        let expected = Result::Ok(Roll::new(1, 20, -3, Condition::Normal).unwrap());
        let actual = Roll::parse("1d20 - 3");

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_with_and_advantage() {
        let expected = Result::Ok(Roll::new(1, 20, 0, Condition::Advantage).unwrap());
        let actual = Roll::parse("1d20 with advantage");

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_with_and_disadvantage() {
        let expected = Result::Ok(Roll::new(1, 20, 0, Condition::Disadvantage).unwrap());
        let actual = Roll::parse("1d20 with disadvantage");

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_with_modifier_and_advantage() {
        let expected = Result::Ok(Roll::new(1, 20, 3, Condition::Advantage).unwrap());
        let actual = Roll::parse("1d20 + 3 with advantage");

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_with_modifier_and_disadvantage() {
        let expected = Result::Ok(Roll::new(1, 20, 3, Condition::Disadvantage).unwrap());
        let actual = Roll::parse("1d20 + 3 with disadvantage");

        assert_eq!(actual, expected);
    }
}
