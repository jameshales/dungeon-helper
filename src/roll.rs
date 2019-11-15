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

/// A dice roll that might occur in Dungeons and Dragons 5th edition.
///
/// A dice roll involves rolling a number of dice, each with a number of sides. The sum of the
/// rolled dice is added to the modifier, which may be positive or negative. The roll may have
/// advantage or disadvantage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Roll {
    rolls: usize,
    sides: i32,
    modifier: i32,
    condition: Option<Condition>,
}

/// Determines the conditions under which a roll occurs - advantage, disadvantage, or normal.
///
/// A roll with advantage involves performing the roll twice and taking the highest result, whereas
/// a roll with disadvantage involves performing the roll twice and taking the lowest result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Condition {
    Advantage,
    Disadvantage,
}

/// The detailed result of a dice roll.
///
/// In addition to the numerical result itself, it includes the individual die values, the
/// modifier, and whether the roll was a critical success or failure, so that this information can
/// be presented to the user.
#[derive(Debug, Eq, PartialEq)]
pub struct RollResult {
    result: i32,
    dice: Vec<i32>,
    modifier: i32,
    critical: Option<Critical>,
}

/// Determines whether the result of a roll was a critical success or failure.
///
/// An ability or skill check involves rolling a single D20 with a modifier. If the result without
/// the modifier is 20 then the roll was a critical success, and if it is 1 then the roll was a
/// critical failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Critical {
    Success,
    Failure,
}

impl fmt::Display for RollResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.result).and({
            if self.dice.len() > 1 || self.modifier != 0 {
                let mut iter = self.dice.iter().take(MAXIMUM_ROLLS_DISPLAY);
                iter.next().map_or(Ok(()), |head| {
                    iter.fold(write!(f, " ({}", head), |result, die| {
                        result.and(write!(f, " + {}", die))
                    })
                    .and(if self.dice.len() > MAXIMUM_ROLLS_DISPLAY {
                        write!(f, " + …")
                    } else {
                        Ok(())
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
                Ok(())
            }
        })
    }
}

/// The detailed results of a dice roll, optionally with the condition of advantage or
/// disadvantage.
///
/// When the dice roll is under advantage or disadvantage, two dice rolls are performed, and the
/// highest or lowest result respectively is chosen. The results of both rolls are included, so
/// that they can be displayed to the user. The primary result is the overall result of the
/// conditional roll, and the optional secondary result is for the roll that is ignored.
#[derive(Debug, Eq, PartialEq)]
pub struct ConditionalRollResult {
    primary: RollResult,
    secondary: Option<RollResult>,
}

impl fmt::Display for ConditionalRollResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.primary)
            .and(
                self.secondary
                    .as_ref()
                    .map_or(Ok(()), |secondary| write!(f, " / ~~{}~~", secondary)),
            )
            .and(
                self.primary
                    .critical
                    .map_or(Ok(()), |critical| match critical {
                        Critical::Failure => write!(f, " — Critical Failure 😰"),
                        Critical::Success => write!(f, " — Critical Success 🤩"),
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::RollsNonPositive => write!(f, "Must roll at least one die"),
            Error::RollsTooGreat => write!(f, "Must roll no more than 100 dice."),
            Error::SidesNonPositive => write!(f, "Dice must have at least one side."),
            Error::SidesTooGreat => write!(f, "Dice must have no more than 100 sides."),
        }
    }
}

/// Represents an error that might occur when parsing a roll from a String.
#[derive(Debug, Eq, PartialEq)]
pub enum ParserError {
    InvalidSyntax,
    InvalidValue(Error),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::InvalidSyntax => write!(f, "Invalid syntax."),
            ParserError::InvalidValue(e) => e.fmt(f),
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
        condition: Option<Condition>,
    ) -> Result<Roll, Error> {
        if rolls <= 0 {
            Err(Error::RollsNonPositive)
        } else if rolls > MAXIMUM_ROLLS {
            Err(Error::RollsTooGreat)
        } else if sides <= 0 {
            Err(Error::SidesNonPositive)
        } else if sides > MAXIMUM_SIDES {
            Err(Error::SidesTooGreat)
        } else {
            Ok(Roll {
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
                let condition = captures.get(5).and_then(|m| match m.as_str() {
                    "advantage" => Some(Condition::Advantage),
                    "disadvantage" => Some(Condition::Disadvantage),
                    _ => None,
                });

                rolls.and_then(|rolls| sides.map(|sides| (rolls, sides, modifier, condition)))
            })
            .ok_or(ParserError::InvalidSyntax)
            .and_then(|(rolls, sides, modifier, condition)| {
                Roll::new(rolls, sides, modifier, condition)
                    .map_err(|e| ParserError::InvalidValue(e))
            })
    }

    /// Roll the dice described by this roll, with any modifier
    pub fn roll<R: Rng + ?Sized>(&self, rng: &mut R) -> ConditionalRollResult {
        let first = self.roll_once(rng);
        let second = self.roll_once(rng);
        match self.condition {
            Some(Condition::Advantage) => {
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
            Some(Condition::Disadvantage) => {
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
            None => ConditionalRollResult {
                primary: first,
                secondary: None,
            },
        }
    }

    /// Roll the dice once, not taking into acccount advantage or disadvantage. This is repeated in
    /// order to perform a roll with advanatge or disadvantage.
    fn roll_once<R: Rng + ?Sized>(&self, rng: &mut R) -> RollResult {
        let dice = self.roll_once_component(rng);
        let sum: i32 = dice.iter().sum();
        let result = sum + self.modifier;
        let critical = if self.rolls == 1 && self.sides == 20 {
            if sum == 1 {
                Some(Critical::Failure)
            } else if sum == 20 {
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

    fn roll_once_component<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec<i32> {
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
                Ok(())
            })
            .and(match self.condition {
                Some(Condition::Advantage) => write!(f, " with advantage"),
                Some(Condition::Disadvantage) => write!(f, " with disadvantage"),
                None => Ok(()),
            })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand_pcg::Pcg32;

    #[test]
    fn test_roll_rolls_non_positive() {
        let expected = Err(Error::RollsNonPositive);
        let actual = Roll::new(0, 20, 0, None);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_roll_sides_non_positive() {
        let expected = Err(Error::SidesNonPositive);
        let actual = Roll::new(1, 0, 0, None);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_roll_rolls_too_great() {
        let expected = Err(Error::RollsTooGreat);
        let actual = Roll::new(101, 20, 0, None);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_roll_sides_too_great() {
        let expected = Err(Error::SidesTooGreat);
        let actual = Roll::new(1, 101, 0, None);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_display_roll_simple() {
        let roll = Roll::new(1, 20, 0, None).unwrap();

        let expected = "1d20";
        let actual = roll.to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_display_roll_with_positive_modifier() {
        let roll = Roll::new(1, 20, 3, None).unwrap();

        let expected = "1d20 + 3";
        let actual = roll.to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_display_roll_with_negative_modifier() {
        let roll = Roll::new(1, 20, -3, None).unwrap();

        let expected = "1d20 - 3";
        let actual = roll.to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_display_roll_with_advantage() {
        let roll = Roll::new(1, 20, 0, Some(Condition::Advantage)).unwrap();

        let expected = "1d20 with advantage";
        let actual = roll.to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_display_roll_with_disadvantage() {
        let roll = Roll::new(1, 20, 0, Some(Condition::Disadvantage)).unwrap();

        let expected = "1d20 with disadvantage";
        let actual = roll.to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_display_roll_with_modifier_and_advantage() {
        let roll = Roll::new(1, 20, 3, Some(Condition::Advantage)).unwrap();

        let expected = "1d20 + 3 with advantage";
        let actual = roll.to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_roll_simple() {
        let expected = Ok(Roll::new(1, 20, 0, None).unwrap());
        let actual = Roll::parse("1d20");

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_roll_with_positive_modifier() {
        let expected = Ok(Roll::new(1, 20, 3, None).unwrap());
        let actual = Roll::parse("1d20 + 3");

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_roll_with_negative_modifier() {
        let expected = Ok(Roll::new(1, 20, -3, None).unwrap());
        let actual = Roll::parse("1d20 - 3");

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_roll_with_and_advantage() {
        let expected = Ok(Roll::new(1, 20, 0, Some(Condition::Advantage)).unwrap());
        let actual = Roll::parse("1d20 with advantage");

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_roll_with_and_disadvantage() {
        let expected = Ok(Roll::new(1, 20, 0, Some(Condition::Disadvantage)).unwrap());
        let actual = Roll::parse("1d20 with disadvantage");

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_roll_with_modifier_and_advantage() {
        let expected = Ok(Roll::new(1, 20, 3, Some(Condition::Advantage)).unwrap());
        let actual = Roll::parse("1d20 + 3 with advantage");

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_roll_with_modifier_and_disadvantage() {
        let expected = Ok(Roll::new(1, 20, 3, Some(Condition::Disadvantage)).unwrap());
        let actual = Roll::parse("1d20 + 3 with disadvantage");

        assert_eq!(actual, expected);
    }

    struct RollDistribution {
        roll: Roll,
    }

    impl Distribution<ConditionalRollResult> for RollDistribution {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ConditionalRollResult {
            self.roll.roll(rng)
        }
    }

    fn validate_result(roll: &Roll, result: &RollResult) -> () {
        assert!(
            result.result >= roll.rolls as i32 + roll.modifier,
            "Result is less than the number of rolls"
        );
        assert!(
            result.result <= (roll.rolls as i32) * roll.sides + roll.modifier,
            "Result is greater than the product of the number of rolls and the number of sides"
        );
        assert!(
            result.modifier == roll.modifier,
            "Result modifier is not equal to the roll modifier"
        );
        let _ = result.dice.iter().map(|die| {
            assert!(*die >= 1, "Die is less than 1");
            assert!(
                *die <= roll.sides,
                "Die is greater than the number of sides"
            );
            assert!(
                *die <= result.result - roll.modifier,
                "Die is greater than the result minus the modifier"
            )
        });
        assert!(
            (result.critical == Some(Critical::Success))
                == (result.result == 20 + roll.modifier && roll.rolls == 1 && roll.sides == 20),
            "Result is 20 but not a critical success"
        );
        assert!(
            (result.critical == Some(Critical::Failure))
                == (result.result == 1 + roll.modifier && roll.rolls == 1 && roll.sides == 20),
            "Result is 1 but not a critical failure"
        );
    }

    fn validate_conditional_result(roll: &Roll, result: &ConditionalRollResult) -> () {
        validate_result(roll, &result.primary);
        let _ = result.secondary.as_ref().map_or_else(
            || {
                assert!(
                    roll.condition.is_none(),
                    "Secondary roll is empty but condition is not"
                )
            },
            |secondary| {
                validate_result(roll, &secondary);
                assert!(
                    (!(roll.condition == Some(Condition::Advantage))
                        || result.primary.result >= secondary.result),
                    "Condition is advantage but secondary result is larger"
                );
                assert!(
                    (!(roll.condition == Some(Condition::Disadvantage))
                        || result.primary.result <= secondary.result),
                    "Condition is disadvantage but secondary result is smaller"
                );
            },
        );
    }

    #[test]
    fn test_roll_1d20() {
        let mut rng = Pcg32::new(0, 0);

        let roll = Roll::new(1, 20, 0, None).unwrap();

        let distribution = RollDistribution { roll };

        let _ = distribution
            .sample_iter(&mut rng)
            .take(100)
            .map(|result| validate_conditional_result(&roll, &result));
    }

    #[test]
    fn test_roll_3d20plus5() {
        let mut rng = Pcg32::new(0, 0);

        let roll = Roll::new(3, 20, 5, None).unwrap();

        let distribution = RollDistribution { roll };

        let _ = distribution
            .sample_iter(&mut rng)
            .take(100)
            .map(|result| validate_conditional_result(&roll, &result));
    }

    #[test]
    fn test_roll_5d8plus3() {
        let mut rng = Pcg32::new(0, 0);

        let roll = Roll::new(5, 8, 3, None).unwrap();

        let distribution = RollDistribution { roll };

        let _ = distribution
            .sample_iter(&mut rng)
            .take(100)
            .map(|result| validate_conditional_result(&roll, &result));
    }

    #[test]
    fn test_display_roll_result_simple() {
        let result = RollResult {
            result: 15,
            dice: vec![15],
            modifier: 0,
            critical: None,
        };
        let expected = "15";
        let actual = result.to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_display_roll_result_with_critical_success() {
        let result = RollResult {
            result: 20,
            dice: vec![20],
            modifier: 0,
            critical: Some(Critical::Success),
        };
        let expected = "20";
        let actual = result.to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_display_roll_result_with_critical_failure() {
        let result = RollResult {
            result: 1,
            dice: vec![1],
            modifier: 0,
            critical: Some(Critical::Failure),
        };
        let expected = "1";
        let actual = result.to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_display_roll_result_with_modifier() {
        let result = RollResult {
            result: 12,
            dice: vec![9],
            modifier: 3,
            critical: None,
        };
        let expected = "12 (9 + __3__)";
        let actual = result.to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_display_roll_result_with_multiple() {
        let result = RollResult {
            result: 18,
            dice: vec![8, 7],
            modifier: 3,
            critical: None,
        };
        let expected = "18 (8 + 7 + __3__)";
        let actual = result.to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_display_roll_result_with_maximum_multiples() {
        let result = RollResult {
            result: 59,
            dice: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            modifier: 4,
            critical: None,
        };
        let expected = "59 (1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10 + __4__)";
        let actual = result.to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_display_roll_result_with_too_many_multiples() {
        let result = RollResult {
            result: 95,
            dice: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
            modifier: 4,
            critical: None,
        };
        let expected = "95 (1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10 + … + __4__)";
        let actual = result.to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_display_conditional_roll_result_simple() {
        let result = ConditionalRollResult {
            primary: RollResult {
                result: 15,
                dice: vec![15],
                modifier: 0,
                critical: None,
            },
            secondary: None,
        };
        let expected = "15";
        let actual = result.to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_display_conditional_roll_result_with_multiple() {
        let result = ConditionalRollResult {
            primary: RollResult {
                result: 17,
                dice: vec![17],
                modifier: 0,
                critical: None,
            },
            secondary: Some(RollResult {
                result: 13,
                dice: vec![13],
                modifier: 0,
                critical: None,
            }),
        };
        let expected = "17 / ~~13~~";
        let actual = result.to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_display_conditional_roll_result_with_critical_success() {
        let result = ConditionalRollResult {
            primary: RollResult {
                result: 20,
                dice: vec![20],
                modifier: 0,
                critical: Some(Critical::Success),
            },
            secondary: None,
        };
        let expected = "20 — Critical Success 🤩";
        let actual = result.to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_display_conditional_roll_result_with_critical_failure() {
        let result = ConditionalRollResult {
            primary: RollResult {
                result: 1,
                dice: vec![1],
                modifier: 0,
                critical: Some(Critical::Failure),
            },
            secondary: None,
        };
        let expected = "1 — Critical Failure 😰";
        let actual = result.to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_display_conditional_roll_result_with_multiple_and_critical_success() {
        let result = ConditionalRollResult {
            primary: RollResult {
                result: 20,
                dice: vec![20],
                modifier: 0,
                critical: Some(Critical::Success),
            },
            secondary: Some(RollResult {
                result: 14,
                dice: vec![14],
                modifier: 0,
                critical: None,
            }),
        };
        let expected = "20 / ~~14~~ — Critical Success 🤩";
        let actual = result.to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_display_conditional_roll_result_with_multiple_and_critical_failure() {
        let result = ConditionalRollResult {
            primary: RollResult {
                result: 1,
                dice: vec![1],
                modifier: 0,
                critical: Some(Critical::Failure),
            },
            secondary: Some(RollResult {
                result: 18,
                dice: vec![18],
                modifier: 0,
                critical: None,
            }),
        };
        let expected = "1 / ~~18~~ — Critical Failure 😰";
        let actual = result.to_string();

        assert_eq!(actual, expected);
    }
}
