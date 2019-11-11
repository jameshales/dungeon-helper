use crate::character::{CharacterAttribute, CharacterAttributeUpdate};
use crate::character_roll::CharacterRoll;
use crate::roll::Roll;
use regex::Regex;
use std::convert::identity;

pub enum Command {
    Error(String),
    CharacterRoll(crate::character_roll::CharacterRoll),
    Help,
    Roll(crate::roll::Roll),
    Set(CharacterAttributeUpdate),
    Show(CharacterAttribute),
    ShowAbilities,
    ShowSkills,
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
        } else if let Some(captures) = ROLL_COMMAND_REGEX.captures(&command) {
            let roll_command = captures.get(1).map_or("", |m| m.as_str()).to_string();
            Some(
                Roll::parse(&roll_command)
                    .map(|roll| Command::Roll(roll))
                    .map_err(|error| Command::Error(error.message().to_string()))
                    .or(CharacterRoll::parse(&roll_command)
                        .map(|roll| Command::CharacterRoll(roll))
                        .ok_or(Command::Error("Invalid character roll?".to_string())))
                    .unwrap_or_else(identity),
            )
        } else if let Some(captures) = SET_COMMAND_REGEX.captures(&command) {
            let set_command = captures.get(1).map_or("", |m| m.as_str()).to_string();
            Some(
                CharacterAttributeUpdate::parse(&set_command)
                    .map(|attribute| Command::Set(attribute))
                    .unwrap_or(Command::Error("Invalid set command.".to_string())),
            )
        } else {
            None
        }
    }
}
