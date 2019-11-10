use regex::Regex;
use crate::character::CharacterAttribute;
use crate::character_roll::CharacterRoll;
use crate::roll::Roll;
use std::convert::identity;

pub enum Command {
    Error { message: String },
    CharacterRoll { roll: CharacterRoll },
    Help,
    Increment,
    Roll { roll: Roll },
    Set { attribute: CharacterAttribute },
    ShowAbilities,
}

impl Command {
    pub fn parse(command: &str) -> Option<Command> {
        lazy_static! {
            static ref ROLL_COMMAND_REGEX: Regex = Regex::new(r"^!(?:r|roll) +(.*)$").unwrap();
            static ref SET_COMMAND_REGEX: Regex = Regex::new(r"^!set +(.*)$").unwrap();
        }

        if command == "!abilities" {
            Some(Command::ShowAbilities)
        } else if command == "!help" {
            Some(Command::Help)
        } else if command == "!increment" {
            Some(Command::Increment)
        } else if let Some(captures) = ROLL_COMMAND_REGEX.captures(&command) {
            let roll_command = captures.get(1).map_or("", |m| m.as_str()).to_string();
            Some(
                Roll::parse(&roll_command)
                    .map(|roll| Command::Roll { roll })
                    .map_err(|error| Command::Error {
                        message: error.message().to_string(),
                    })
                    .or(
                        CharacterRoll::parse(&roll_command)
                            .map(|roll| Command::CharacterRoll { roll })
                            .ok_or(Command::Error { message: "Invalid character roll?".to_string() })
                    )
                    .unwrap_or_else(identity),
            )
        } else if let Some(captures) = SET_COMMAND_REGEX.captures(&command) {
            let set_command = captures.get(1).map_or("", |m| m.as_str()).to_string();
            Some(
                CharacterAttribute::parse(&set_command)
                    .map(|attribute| Command::Set { attribute })
                    .unwrap_or(Command::Error { message: "Invalid set command.".to_string() })
            )
        } else {
            None
        }
    }
}
