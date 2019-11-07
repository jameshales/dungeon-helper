use rand::Rng;
use rand::distributions::{Distribution, Uniform};
use std::cmp::{max, min};
use std::fmt;

pub enum Condition {
    Advantage,
    Normal,
    Disadvantage
}

pub struct Roll {
    rolls: usize,
    sides: i32,
    modifier: i32,
    condition: Condition
}

impl Roll {
    pub fn new(rolls: usize, sides: i32, modifier: i32, condition: Condition) -> Roll {
        Roll { rolls, sides, modifier, condition }
    }

    pub fn roll<R: Rng>(&self, rng: &mut R) -> i32 {
        match self.condition {
            Condition::Advantage    => max(self.roll_once(rng), self.roll_once(rng)),
            Condition::Normal       => self.roll_once(rng),
            Condition::Disadvantage => min(self.roll_once(rng), self.roll_once(rng)),
        }
    }

    fn roll_once<R: Rng>(&self, rng: &mut R) -> i32 {
        Uniform::new_inclusive(1, self.sides)
            .sample_iter(rng)
            .take(self.rolls)
            .sum::<i32>() + self.modifier
    }
}

impl fmt::Display for Roll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}d{}", self.rolls, self.sides)
            .and(
                if self.modifier > 0 {
                    write!(f, " + {}", self.modifier)
                } else if self.modifier < 0 {
                    write!(f, " - {}", self.modifier.abs())
                } else {
                    Result::Ok(())
                }
            )
            .and(
                match self.condition {
                    Condition::Advantage    => write!(f, " with advantage"),
                    Condition::Normal       => Result::Ok(()),
                    Condition::Disadvantage => write!(f, " with disadvantage")
                }
            )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_simple() {
        let roll = Roll::new(1, 20, 0, Condition::Normal);

        let expected = "1d20";
        let actual = format!("{}", roll);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_display_with_modifier() {
        let roll = Roll::new(1, 20, 3, Condition::Normal);

        let expected = "1d20 + 3";
        let actual = format!("{}", roll);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_display_with_advantage() {
        let roll = Roll::new(1, 20, 0, Condition::Advantage);

        let expected = "1d20 with advantage";
        let actual = format!("{}", roll);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_display_with_disadvantage() {
        let roll = Roll::new(1, 20, 0, Condition::Disadvantage);

        let expected = "1d20 with disadvantage";
        let actual = format!("{}", roll);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_display_with_modifier_and_advantage() {
        let roll = Roll::new(1, 20, 3, Condition::Advantage);

        let expected = "1d20 + 3 with advantage";
        let actual = format!("{}", roll);

        assert_eq!(actual, expected);
    }
}
