use crate::character::{CharacterAttribute, CharacterAttributeUpdate};
use crate::character_roll::CharacterRoll;
use crate::roll::Roll;
use regex::Regex;
use std::convert::identity;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    R2D2Error(r2d2::Error),
    RusqliteError(rusqlite::Error),
    IntentParserError(::failure::Error),
    UnknownIntent(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::R2D2Error(error) => write!(f, "Connection pool error: {}", error),
            Error::RusqliteError(error) => write!(f, "Database error: {}", error),
            Error::IntentParserError(error) => write!(f, "Intent parser error: {}", error),
            Error::UnknownIntent(intent_name) => write!(f, "Unknown intent: {}", intent_name),
        }
    }
}

#[derive(Debug)]
pub enum Command {
    CharacterRoll(crate::character_roll::CharacterRoll),
    Clarification(String),
    Help,
    HelpShorthand,
    Roll(crate::roll::Roll),
    Set(CharacterAttributeUpdate),
    SetBotDisabled,
    SetBotEnabled,
    SetCharactersLocked,
    SetCharactersUnlocked,
    Show(CharacterAttribute),
    ShowError(Error),
    ShowWarning(String),
    ShowAbilities,
    ShowSkills,
}

impl Command {
    pub fn is_admin(&self) -> bool {
        match self {
            Command::SetBotDisabled | Command::SetBotEnabled => true,
            _ => false
        }
    }

    pub fn is_editing(&self) -> bool {
        match self {
            Command::Set(_) => true,
            _ => false
        }
    }

    pub fn parse_shorthand(command: &str) -> Option<Command> {
        lazy_static! {
            static ref ROLL_COMMAND_REGEX: Regex = Regex::new(r"^!(?:r|roll) +(.*)$").unwrap();
        }

        if command == "!help" {
            Some(Command::HelpShorthand)
        } else if let Some(captures) = ROLL_COMMAND_REGEX.captures(&command) {
            let roll_command = captures.get(1).map_or("", |m| m.as_str()).to_string();
            Some(
                Roll::parse(&roll_command)
                    .map(|roll| Command::Roll(roll))
                    .map_err(|error| Command::ShowWarning(error.message().to_string()))
                    .or(CharacterRoll::parse(&roll_command)
                        .map(|roll| Command::CharacterRoll(roll))
                        .ok_or(Command::ShowWarning("Invalid syntax.".to_string()))
                    )
                    .unwrap_or_else(identity)
            )
        } else {
            None
        }
    }
}
