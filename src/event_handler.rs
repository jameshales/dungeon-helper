use crate::attack_roll::AttackRoll;
use crate::channel::Channel;
use crate::character::Character;
use crate::character_roll::CharacterRoll;
use crate::command;
use crate::command::{Command, CommandResult};
use crate::error::Error;
use crate::intent_logger::log_intent_result;
use crate::response::Response;
use crate::roll::{ConditionalRoll, Critical};
use log::{error, info};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use snips_nlu_lib::SnipsNluEngine;
use snips_nlu_ontology::IntentParserResult;
use std::borrow::Cow;
use std::convert::identity;
use std::sync::RwLock;
use symspell::{SymSpell, UnicodeStringStrategy};

use serenity::{
    model::{
        channel::Message,
        gateway::Ready,
        id::{ChannelId, UserId},
    },
    prelude::*,
};

const CHARACTER_NOT_FOUND_WARNING_TEXT: &str =
    "Couldn't find any attributes for character. Try setting some ability scores and a character level first.";

const ABILITY_NOT_SET_WARNING_TEXT: &str =
    "Couldn't find required ability scores for character. Try setting some ability scores and a character level first.";

enum Action {
    IgnoreChannelDisabled,
    IgnoreCommandMissing,
    IgnoreOwnMessage,
    Respond(Response),
}

pub struct Handler {
    pub bot_id: RwLock<Option<String>>,
    pub engine: SnipsNluEngine,
    pub pool: Pool<SqliteConnectionManager>,
    pub symspell: SymSpell<UnicodeStringStrategy>,
}

impl Handler {
    fn get_command(
        &self,
        engine: &SnipsNluEngine,
        symspell: &SymSpell<UnicodeStringStrategy>,
        message: &Message,
        dice_only: bool,
    ) -> Option<Result<CommandResult, command::Error>> {
        let content = &message.content.trim();
        self.bot_id
            .try_read()
            .ok()
            .and_then(|bot_id| {
                bot_id.as_ref().map(|bot_id| {
                    Command::parse(engine, symspell, content, Some(&bot_id), dice_only)
                })
            })
            .unwrap_or_else(|| Command::parse(engine, symspell, content, None, dice_only))
    }

    fn get_action(
        &self,
        command_result: Option<Result<CommandResult, command::Error>>,
        channel: &Channel,
        message: &Message,
        is_admin: bool,
        is_private: bool,
    ) -> Action {
        command_result.map_or(Action::IgnoreCommandMissing, |command_result| {
            command_result
                .map(|command_result| {
                    let command = match command_result {
                        CommandResult::Shorthand(command) => command,
                        CommandResult::NaturalLanguage(command, intent_result, corrected) => {
                            self.log_intent_result(&message, &intent_result, corrected.as_deref());
                            command
                        }
                    };
                    match command {
                        Ok(command) => {
                            if !is_admin && !channel.enabled {
                                Action::IgnoreChannelDisabled
                            } else if is_private && !command.is_private() {
                                Action::Respond(Response::Warning(format!("It looks like you're trying to {}. You can't do that in a private message.", command.description())))
                            } else {
                                Action::Respond(self.run_command(
                                    command,
                                    message.channel_id,
                                    message.author.id,
                                ))
                            }
                        }
                        Err(error) => Action::Respond(error.into_response()),
                    }
                })
                .unwrap_or_else(|error| Action::Respond(error.into_response()))
        })
    }

    fn run_command(&self, command: Command, channel_id: ChannelId, author_id: UserId) -> Response {
        match command {
            Command::AttackRoll(roll) => self.attack_roll(&roll, channel_id, author_id),
            Command::CharacterRoll(roll) => self.character_roll(&roll, channel_id, author_id),
            Command::Help => Handler::help(),
            Command::HelpShorthand => Handler::help_shorthand(),
            Command::Roll(roll) => Handler::roll(roll),
        }
    }

    fn log_intent_result(
        &self,
        message: &Message,
        intent_result: &IntentParserResult,
        corrected: Option<&str>,
    ) {
        self.pool
            .get()
            .map_err(|error| error!(target: "dungeon-helper", "Error obtaining database connection. Message ID: {}; Error: {}", message.id, error))
            .and_then(|mut connection| {
                log_intent_result(&mut connection, message, intent_result, corrected)
                    .map_err(|error|
                        error!(target: "dungeon-helper", "Error logging intent result. Message ID: {}; Error: {}", message.id, error)
                    )
            })
            .unwrap_or(())
    }

    fn attack_roll(
        &self,
        attack_roll: &AttackRoll,
        channel_id: ChannelId,
        author_id: UserId,
    ) -> Response {
        self.pool
            .get()
            .map_err(|error| Response::Error(Error::R2D2Error(error)))
            .and_then(|connection| {
                Character::get(&connection, channel_id, author_id)
                    .map_err(|error| Response::Error(Error::RusqliteError(error)))
                    .and_then(|character| {
                        character.map_or(
                            Err(Response::Warning(
                                CHARACTER_NOT_FOUND_WARNING_TEXT.to_owned(),
                            )),
                            |character| match attack_roll {
                                AttackRoll::Weapon(attack_roll) => {
                                    Character::has_weapon_proficiency(
                                        &connection,
                                        channel_id,
                                        author_id,
                                        attack_roll.weapon,
                                        attack_roll.weapon.to_weapon().category,
                                    )
                                    .map(|proficiency| (character, proficiency))
                                    .map_err(|error| Response::Error(Error::RusqliteError(error)))
                                }
                                _ => Ok((character, false)),
                            },
                        )
                    })
            })
            .and_then(|(character, proficiency)| {
                let strength = character.strength().map(|a| a.modifier);
                let dexterity = character.dexterity().map(|a| a.modifier);
                let proficiency_bonus = character.proficiency_bonus();
                let mut rng = rand::thread_rng();
                let to_hit_roll = attack_roll
                    .to_attack_roll(
                        strength,
                        dexterity,
                        proficiency_bonus,
                        proficiency,
                        character.martial_arts(),
                    )
                    .ok_or_else(|| Response::Warning(ABILITY_NOT_SET_WARNING_TEXT.to_owned()))?;
                let to_hit_result = to_hit_roll.roll(&mut rng);
                let critical_hit = to_hit_result.critical() == Some(Critical::Success);
                let damage_roll = attack_roll
                    .to_damage_roll(
                        strength,
                        dexterity,
                        critical_hit,
                        character.martial_arts_damage_die(),
                    )
                    .ok_or_else(|| Response::Warning(ABILITY_NOT_SET_WARNING_TEXT.to_owned()))?;
                let damage_result = damage_roll.roll(&mut rng);
                Ok((to_hit_roll, to_hit_result, damage_roll, damage_result))
            })
            .map(
                |(to_hit_roll, to_hit_result, damage_roll, damage_result)| Response::AttackRoll {
                    attack_name: attack_roll.get_name(),
                    attack_handedness: attack_roll.get_handedness(),
                    to_hit_roll,
                    to_hit_result,
                    damage_roll,
                    damage_result,
                },
            )
            .unwrap_or_else(identity)
    }

    fn character_roll(
        &self,
        character_roll: &CharacterRoll,
        channel_id: ChannelId,
        author_id: UserId,
    ) -> Response {
        self.pool
            .get()
            .map_err(|error| Response::Error(Error::R2D2Error(error)))
            .and_then(|connection| {
                Character::get(&connection, channel_id, author_id)
                    .map_err(|error| Response::Error(Error::RusqliteError(error)))
            })
            .and_then(|character| {
                character
                    .ok_or_else(|| Response::Warning(CHARACTER_NOT_FOUND_WARNING_TEXT.to_owned()))
            })
            .and_then(|character| {
                character_roll
                    .to_roll(&character)
                    .ok_or_else(|| Response::Warning(ABILITY_NOT_SET_WARNING_TEXT.to_owned()))
            })
            .map(|roll| {
                let mut rng = rand::thread_rng();
                let result = roll.roll(&mut rng);
                Response::CharacterRoll {
                    check: character_roll.check,
                    roll,
                    result,
                }
            })
            .unwrap_or_else(identity)
    }

    fn help() -> Response {
        Response::Help(
            "Try typing the following:\n\
             • \"Roll three d8s\"\n\
             • \"Throw two twelve-sided dice\"\n\
             • \"Do a strength check with advantage\"\n\
             • \"Perform a wisdom saving throw\"\n\
             • \"Try a stealth roll with disadvantage\"\n\
             • \"Roll for initiative\"\n\
             There are also short-hand commands you can use. Type \"!help\" for more info."
                .to_owned(),
        )
    }

    fn help_shorthand() -> Response {
        Response::Help(
            "Try typing the following:\n\
             • \"!r 3d8\"\n\
             • \"!r 2d12+3\"\n\
             • \"!r strength with advantage\"\n\
             • \"!r wisdom saving throw\"\n\
             • \"!r stealth with disadvantage\"\n\
             • \"!r initiative\"\n\
             There are also natural language commands you can use. Type \"help\" for more info."
                .to_owned(),
        )
    }

    fn roll(roll: ConditionalRoll) -> Response {
        let mut rng = rand::thread_rng();
        let result = roll.roll(&mut rng);
        Response::DiceRoll { roll, result }
    }

    fn get_channel(&self, channel_id: ChannelId) -> Channel {
        self.pool
            .get()
            .ok()
            .and_then(|connection|
                Channel::get(&connection, channel_id)
                    .map_err(|error| error!(target: "dungeon-helper", "Error retrieving channel: Channel ID: {}; Error: {}", channel_id.to_string(), error))
                    .ok()
                    .and_then(identity)
            )
            .unwrap_or(
                Channel {
                    enabled: false,
                    locked: false,
                    dice_only: false,
                }
            )
    }
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, message: Message) {
        info!(target: "dungeon-helper", "Received message. Message ID: {}; Content: {}", message.id, message.content.escape_debug());
        let action = if message.is_own(&ctx.cache) {
            // Don't respond to our own messages, this may cause an infinite loop
            Action::IgnoreOwnMessage
        } else {
            let channel = self.get_channel(message.channel_id);
            let is_admin = message.member(&ctx.cache).map_or(true, |member| {
                member
                    .permissions(&ctx.cache)
                    .ok()
                    .map_or(false, |permissions| permissions.administrator())
            });
            let is_private = message.is_private();
            let command_result = self.get_command(
                &self.engine,
                &self.symspell,
                &message,
                // Private channels are implicitly dice only, no need to @me
                channel.dice_only || is_private,
            );
            if let Some(command_result) = command_result.as_ref() {
                match command_result {
                    Ok(CommandResult::NaturalLanguage(Ok(command), _, corrected)) => {
                        info!(target: "dungeon-helper", "Parsed natural language command successfully. Message ID: {}; Command: {:?}; Corrected Message: {}", message.id, command, corrected.as_deref().unwrap_or(""))
                    }
                    Ok(CommandResult::NaturalLanguage(Err(error), _, corrected)) => {
                        info!(target: "dungeon-helper", "Error parsing natural language command. Message ID: {}; Corrected Message: {}; Error: {:}", message.id, corrected.as_deref().unwrap_or(""), error)
                    }
                    Ok(CommandResult::Shorthand(Err(error))) => {
                        info!(target: "dungeon-helper", "Error parsing shorthand command. Message ID: {}; Command: {:?}", message.id, error)
                    }
                    Ok(CommandResult::Shorthand(Ok(command))) => {
                        info!(target: "dungeon-helper", "Parsed shorthand command successfully. Message ID: {}; Command: {:?}", message.id, command)
                    }
                    Err(error) => {
                        info!(target: "dungeon-helper", "Error parsing command. Message ID: {}; Error: {}", message.id, error)
                    }
                }
            };
            self.get_action(command_result, &channel, &message, is_admin, is_private)
        };
        match action {
            Action::IgnoreChannelDisabled => {
                info!(target: "dungeon-helper", "Ignoring command because Dungeon Helper is disabled in current channel. Message ID: {}", message.id);
            }
            Action::IgnoreCommandMissing => {
                info!(target: "dungeon-helper", "Ignoring message because it contains no command. Message ID: {}", message.id);
            }
            Action::IgnoreOwnMessage => {
                info!(target: "dungeon-helper", "Ignoring message because it was sent by us. Message ID: {}", message.id);
            }
            Action::Respond(response) => {
                if let Response::Error(error) = &response {
                    error!(target: "dungeon-helper", "Error processing command. Message ID: {}; Error = {:?}", message.id, error);
                };
                let author_nick = match message.author_nick(&ctx.http) {
                    Some(nick) => Cow::Owned(nick),
                    None => Cow::Borrowed(&message.author.name),
                };
                let result = message.channel_id.send_message(&ctx.http, |builder| {
                    response.to_message(&author_nick, &message, builder)
                });
                match result {
                    Ok(sent_message) => {
                        info!(target: "dungeon-helper", "Sent message. Message ID: {}; Sent Message ID: {}; Content: {}", message.id, sent_message.id, sent_message.content.escape_debug());

                        if response.is_roll() {
                            let delete_result = message.delete(&ctx.http);
                            match delete_result {
                                Ok(()) => {
                                    info!(target: "dungeon-helper", "Deleted user message. Message ID: {}", message.id)
                                }
                                Err(error) => {
                                    error!(target: "dungeon-helper", "Error deleting message. Message ID: {}; Error: {:?}", message.id, error)
                                }
                            }
                        }
                    }
                    Err(error) => {
                        error!(target: "dungeon-helper", "Error sending message. Message ID: {}; Error: {:?}", message.id, error)
                    }
                }
            }
        };
    }

    fn ready(&self, _: Context, ready: Ready) {
        let mut bot_id = self
            .bot_id
            .write()
            .expect("RwLock for bot_id has been poisoned");
        *bot_id = Some(ready.user.id.to_string());
        info!(target: "dungeon-helper", "{} is connected!", ready.user.name);
    }
}
