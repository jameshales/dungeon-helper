use crate::channel::Channel;
use crate::character::{
    Ability, Character, CharacterAttribute, CharacterAttributeUpdate, SavingThrow, Skill,
};
use crate::character_roll::CharacterRoll;
use crate::command::{Command, Error};
use crate::intent_logger::log_intent_result;
use crate::intent_parser::parse_intent_result;
use crate::roll::Roll;
use log::{error, info};
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use regex::Regex;
use rusqlite::Result as RusqliteResult;
use snips_nlu_lib::SnipsNluEngine;
use snips_nlu_ontology::IntentParserResult;
use std::convert::identity;
use std::sync::RwLock;

use serenity::{
    model::{
        channel::Message,
        gateway::Ready,
        id::{ChannelId, MessageId, UserId},
    },
    prelude::*,
};

const CHARACTER_NOT_FOUND_WARNING_TEXT: &str =
    "Couldn't find any attributes for character. Try setting some ability scores and a character level first.";

const ABILITY_NOT_SET_WARNING_TEXT: &str =
    "Couldn't find required ability scores for character. Try setting some ability scores and a character level first.";

enum Response {
    Clarification(String),
    DiceRoll(String),
    Error(Error),
    Help(String),
    Show(String),
    Update(String),
    Warning(String),
}

impl Response {
    pub fn as_str(&self, author_id: &UserId, message_id: &MessageId) -> String {
        match self {
            Response::Clarification(message) => format!("📎 <@{}> {}", author_id, message),
            Response::DiceRoll(message) => format!("🎲 <@{}> {}", author_id, message),
            Response::Error(error) => {
                error!(target: "dungeon-helper", "Error processing command. Message ID: {}; Error = {:?}", message_id, error);
                format!(
                    "💥 <@{}> **Error:** A technical error has occurred. Reference ID: {}",
                    author_id, message_id
                )
            }
            Response::Help(message) => format!("🎱 <@{}> {}", author_id, message),
            Response::Show(message) => format!("<@{}> {}", author_id, message),
            Response::Update(message) => format!("💾 <@{}> {}", author_id, message),
            Response::Warning(message) => format!("⚠️ <@{}> {}", author_id, message),
        }
    }
}

pub struct Handler {
    pub bot_id: RwLock<Option<String>>,
    pub engine: SnipsNluEngine,
    pub pool: Pool<SqliteConnectionManager>,
}

impl Handler {
    fn get_message_command(&self, message: &Message, dice_only: bool) -> Option<MessageCommand> {
        let content = &message.content.trim();
        Command::parse_shorthand(content)
            .map(MessageCommand::Shorthand)
            .or(self
                .parse_message(content, dice_only)
                .map(|(command, intent_result)| {
                    MessageCommand::NaturalLanguage(command, intent_result)
                }))
    }

    fn get_response(
        &self,
        command: Command,
        channel_id: &ChannelId,
        author_id: &UserId,
    ) -> Response {
        match command {
            Command::CharacterRoll(roll) => {
                self.get_character_roll_response(&roll, channel_id, author_id)
            }
            Command::Clarification(message) => Response::Clarification(message),
            Command::Help => self.help(),
            Command::HelpShorthand => self.help_shorthand(),
            Command::Roll(roll) => Handler::roll(roll),
            Command::Set(attribute) => self.set(&attribute, channel_id, author_id),
            Command::SetChannelDiceOnly(dice_only) => {
                self.set_channel_dice_only(channel_id, dice_only)
            }
            Command::SetChannelEnabled(enabled) => self.set_channel_enabled(channel_id, enabled),
            Command::SetChannelLocked(locked) => self.set_channel_locked(channel_id, locked),
            Command::Show(attribute) => self.show(&attribute, channel_id, author_id),
            Command::ShowError(error) => Response::Error(error),
            Command::ShowWarning(message) => Response::Warning(message),
            Command::ShowAbilities => self.show_abilities(channel_id, author_id),
            Command::ShowSkills => self.show_skills(channel_id, author_id),
        }
    }

    fn parse_message(
        &self,
        message: &str,
        dice_only: bool,
    ) -> Option<(Command, Option<IntentParserResult>)> {
        self.extract_at_message(message, dice_only).map(|at_message| {
            self.engine.parse(at_message.trim(), None, None)
                .map(
                    |result| {
                        let command = parse_intent_result(&result).map_or(
                            Command::Clarification("I'm not sure what you mean. Try asking again with a different or simpler phrasing. Try asking for help to see some examples.".to_string()),
                            identity
                        );
                        (command, Some(result))
                    }
                )
                .unwrap_or_else(|error| (Command::ShowError(Error::IntentParserError(error)), None))
        })
    }

    fn extract_at_message(&self, message: &str, dice_only: bool) -> Option<String> {
        lazy_static! {
            static ref COMMAND_REGEX: Regex = Regex::new(r"^(?:<@!?(\d+)> *)?(.*)$").unwrap();
        }

        let bot_id = self.bot_id.try_read().ok();

        COMMAND_REGEX.captures(&message).and_then(|c| {
            let is_at_message = c.get(1).map_or(false, |m| {
                bot_id.map_or(false, |bot_id| *bot_id == Some(m.as_str().to_string()))
            });
            if dice_only || is_at_message {
                c.get(2).map(|m| m.as_str().to_string())
            } else {
                None
            }
        })
    }

    fn log_intent_result(
        &self,
        message: &Message,
        intent_result: &Option<IntentParserResult>,
    ) -> () {
        self.pool
            .get()
            .map_err(|error| error!(target: "dungeon-helper", "Error obtaining database connection. Message ID: {}; Error: {}", message.id, error))
            .and_then(|mut connection| {
                log_intent_result(&mut connection, message, intent_result)
                    .map_err(|error|
                        error!(target: "dungeon-helper", "Error logging intent result. Message ID: {}; Error: {}", message.id, error)
                    )
            })
            .unwrap_or(())
    }

    fn get_character_roll_response(
        &self,
        character_roll: &CharacterRoll,
        channel_id: &ChannelId,
        author_id: &UserId,
    ) -> Response {
        self.pool
            .get()
            .map_err(|error| Response::Error(Error::R2D2Error(error)))
            .and_then(|connection| {
                Character::get(&connection, channel_id, author_id)
                    .map_err(|_| Response::Warning(CHARACTER_NOT_FOUND_WARNING_TEXT.to_string()))
            })
            .and_then(|character| {
                character_roll
                    .to_roll(&character)
                    .ok_or(Response::Warning(ABILITY_NOT_SET_WARNING_TEXT.to_string()))
            })
            .map(|roll| {
                let mut rng = rand::thread_rng();
                let result = roll.roll(&mut rng);
                Response::DiceRoll(format!(
                    "rolled {} ({}) = {}",
                    character_roll.check, roll, result
                ))
            })
            .unwrap_or_else(identity)
    }

    fn help(&self) -> Response {
        Response::Help(format!(
            "Try typing the following:\n\
             • \"Roll three d8s\"\n\
             • \"Throw two twelve-sided dice\"\n\
             • \"Do a strength check with advantage\"\n\
             • \"Perform a wisdom saving throw\"\n\
             • \"Try a stealth roll with disadvantage\"\n\
             • \"Roll for initiative\"\n\
             There are also short-hand commands you can use. Type \"!help\" for more info."
        ))
    }

    fn help_shorthand(&self) -> Response {
        Response::Help(format!(
            "Try typing the following:\n\
             • \"!r 3d8\"\n\
             • \"!r 2d12+3\"\n\
             • \"!r strength with advantage\"\n\
             • \"!r wisdom saving throw\"\n\
             • \"!r stealth with disadvantage\"\n\
             • \"!r initiative\"\n\
             There are also natural language commands you can use. Type \"help\" for more info.",
        ))
    }

    fn roll(roll: Roll) -> Response {
        let mut rng = rand::thread_rng();
        let result = roll.roll(&mut rng);
        Response::DiceRoll(format!("rolled {} = {}", roll, result))
    }

    fn with_connection<
        F: FnOnce(PooledConnection<SqliteConnectionManager>) -> RusqliteResult<Response>,
    >(
        &self,
        f: F,
    ) -> Response {
        self.pool
            .get()
            .map_err(|error| Response::Error(Error::R2D2Error(error)))
            .and_then(|connection| {
                f(connection).map_err(|error| Response::Error(Error::RusqliteError(error)))
            })
            .unwrap_or_else(identity)
    }

    fn set(
        &self,
        attribute: &CharacterAttributeUpdate,
        channel_id: &ChannelId,
        author_id: &UserId,
    ) -> Response {
        self.with_connection(|mut connection| {
            Character::set_attribute(&mut connection, channel_id, author_id, attribute)
                .map(|_| Response::Update(format!("Set {}", attribute)))
        })
    }

    fn set_channel_dice_only(&self, channel_id: &ChannelId, dice_only: bool) -> Response {
        self.with_connection(|mut connection| {
            Channel::set_dice_only(&mut connection, channel_id, dice_only).map(|_| {
                Response::Update(format!(
                    "Dice only mode is now {} in this channel.",
                    if dice_only { "enabled" } else { "disabled" }
                ))
            })
        })
    }

    fn set_channel_enabled(&self, channel_id: &ChannelId, enabled: bool) -> Response {
        self.with_connection(|mut connection| {
            Channel::set_enabled(&mut connection, channel_id, enabled).map(|_| {
                Response::Update(format!(
                    "Dungeon Helper is now {} in this channel.",
                    if enabled { "enabled" } else { "disabled" }
                ))
            })
        })
    }

    fn set_channel_locked(&self, channel_id: &ChannelId, locked: bool) -> Response {
        self.with_connection(|mut connection| {
            Channel::set_locked(&mut connection, channel_id, locked).map(|_| {
                Response::Update(format!(
                    "Character attributes are now {} in this channel.",
                    if locked { "locked" } else { "unlocked" }
                ))
            })
        })
    }

    fn show(
        &self,
        attribute: &CharacterAttribute,
        channel_id: &ChannelId,
        author_id: &UserId,
    ) -> Response {
        self.with_connection(|connection| {
            Character::get(&connection, channel_id, author_id)
                .map(|character| Response::Show(Handler::show_attribute(&character, attribute)))
        })
    }

    fn show_attribute(character: &Character, attribute: &CharacterAttribute) -> String {
        match attribute {
            CharacterAttribute::Ability(ability) => format!(
                "{} = {}",
                ability.as_str(),
                Handler::format_ability(character.ability(*ability))
            ),
            CharacterAttribute::Initiative => format!(
                "Initiative = {}",
                character
                    .dexterity()
                    .map_or("?".to_string(), |v| format!("{:+}", v.modifier))
            ),
            CharacterAttribute::JackOfAllTrades => format!(
                "Jack of All Trades = {}",
                if character.jack_of_all_trades() {
                    "Yes"
                } else {
                    "No"
                }
            ),
            CharacterAttribute::Level => format!(
                "Level = {}",
                character.level().map_or("?".to_string(), |v| v.to_string())
            ),
            CharacterAttribute::PassiveAbility(ability) => format!(
                "Passive {} = {}",
                ability.as_str(),
                character
                    .passive_ability(*ability)
                    .map_or("?".to_string(), |v| v.to_string())
            ),
            CharacterAttribute::PassiveSkill(skill) => format!(
                "Passive {} = {}",
                skill.as_str(),
                character
                    .passive_skill(*skill)
                    .map_or("?".to_string(), |v| v.to_string())
            ),
            CharacterAttribute::SavingThrow(ability) => format!(
                "{} saving throw = {}",
                ability.as_str(),
                Handler::format_saving_throw(character.saving_throw(*ability))
            ),
            CharacterAttribute::Skill(skill) => format!(
                "{} = {}",
                skill.as_str(),
                Handler::format_skill(character.skill(*skill))
            ),
        }
    }

    fn show_abilities(&self, channel_id: &ChannelId, author_id: &UserId) -> Response {
        self.with_connection(|connection| {
            Character::get(&connection, channel_id, author_id).map(|character| {
                Response::Show(format!(
                    "\n\
                     STR = {}\n\
                     DEX = {}\n\
                     CON = {}\n\
                     INT = {}\n\
                     WIS = {}\n\
                     CHA = {}",
                    Handler::format_ability(character.strength()),
                    Handler::format_ability(character.dexterity()),
                    Handler::format_ability(character.constitution()),
                    Handler::format_ability(character.intelligence()),
                    Handler::format_ability(character.wisdom()),
                    Handler::format_ability(character.charisma()),
                ))
            })
        })
    }

    fn show_skills(&self, channel_id: &ChannelId, author_id: &UserId) -> Response {
        self.with_connection(|connection| {
            Character::get(&connection, channel_id, author_id).map(|character| {
                Response::Show(format!(
                    "\n\
                     Acrobatics = {}\n\
                     Animal Handling = {}\n\
                     Arcana = {}\n\
                     Athletics = {}\n\
                     Deception = {}\n\
                     History = {}\n\
                     Insight = {}\n\
                     Intimidation = {}\n\
                     Investigation = {}\n\
                     Medicine = {}\n\
                     Nature = {}\n\
                     Perception = {}\n\
                     Performance = {}\n\
                     Persuasion = {}\n\
                     Religion = {}\n\
                     Sleight of Hand = {}\n\
                     Stealth = {}\n\
                     Survival = {}",
                    Handler::format_skill(character.acrobatics()),
                    Handler::format_skill(character.animal_handling()),
                    Handler::format_skill(character.arcana()),
                    Handler::format_skill(character.athletics()),
                    Handler::format_skill(character.deception()),
                    Handler::format_skill(character.history()),
                    Handler::format_skill(character.insight()),
                    Handler::format_skill(character.intimidation()),
                    Handler::format_skill(character.investigation()),
                    Handler::format_skill(character.medicine()),
                    Handler::format_skill(character.nature()),
                    Handler::format_skill(character.perception()),
                    Handler::format_skill(character.performance()),
                    Handler::format_skill(character.persuasion()),
                    Handler::format_skill(character.religion()),
                    Handler::format_skill(character.sleight_of_hand()),
                    Handler::format_skill(character.stealth()),
                    Handler::format_skill(character.survival()),
                ))
            })
        })
    }

    fn format_ability(ability: Option<Ability>) -> String {
        ability.map_or("?".to_string(), |a| {
            format!("{:+} ({})", a.modifier, a.score)
        })
    }

    fn format_saving_throw(saving_throw: Option<SavingThrow>) -> String {
        saving_throw.map_or("?".to_string(), |s| {
            format!(
                "{:+} ({})",
                s.modifier,
                if s.proficiency {
                    "Proficient"
                } else {
                    "Normal"
                }
            )
        })
    }

    fn format_skill(skill: Option<Skill>) -> String {
        skill.map_or("?".to_string(), |s| {
            format!("{:+} ({})", s.modifier, s.proficiency.as_str())
        })
    }

    fn get_channel(&self, channel_id: &ChannelId) -> Channel {
        self.pool
            .get()
            .ok()
            .and_then(|mut connection|
                Channel::get(&mut connection, channel_id)
                    .map_err(|error| error!(target: "dungeon-helper", "Error retrieving channel: Channel ID: {}, Error: {}", channel_id.to_string(), error))
                    .ok()
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

enum MessageCommand {
    Shorthand(Command),
    NaturalLanguage(Command, Option<IntentParserResult>),
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, message: Message) {
        if message.is_own(&ctx.cache) {
            // Don't respond to our own messages, this may cause an infinite loop
            info!(target: "dungeon-helper", "Sent message. Message ID: {}; Content: {}", message.id, message.content.escape_debug());
        } else {
            info!(target: "dungeon-helper", "Received message. Message ID: {}; Content: {}", message.id, message.content.escape_debug());
            let channel = self.get_channel(&message.channel_id);
            self.get_message_command(&message, channel.dice_only).map_or_else(
                || info!(target: "dungeon-helper", "Message contains no command. Message ID: {}", message.id),
                |message_command| {
                    let command = match message_command {
                        MessageCommand::Shorthand(command) => {
                            info!(target: "dungeon-helper", "Handling shorthand command. Message ID: {}", message.id);
                            command
                        }
                        MessageCommand::NaturalLanguage(command, intent_result) => {
                            info!(target: "dungeon-helper", "Handling natural language command. Message ID: {}", message.id);
                            self.log_intent_result(&message, &intent_result);
                            command
                        }
                    };
                    info!(target: "dungeon-helper", "Parsed command. Message ID: {}; Command: {:?}", message.id, command);
                    let is_admin = message.member(&ctx.cache).map_or(true, |member| member.permissions(&ctx.cache).ok().map_or(false, |permissions| permissions.administrator()));
                    if !is_admin && !channel.enabled {
                        info!(target: "dungeon-helper", "Ignoring command because Dungeon Helper is disabled in current channel. Message ID: {}", message.id);
                    } else if !is_admin && channel.locked && command.is_editing() {
                        info!(target: "dungeon-helper", "Ignoring command because editing is locked in current channel. Message ID: {}", message.id);
                    } else if !is_admin && command.is_admin() {
                        info!(target: "dungeon-helper", "Ignoring command because it is an admin command and the current user is not an admin. Message ID: {}", message.id);
                    } else {
                        let response = self.get_response(command, &message.channel_id, &message.author.id);
                        if let Err(why) = message
                            .channel_id
                            .say(&ctx.http, response.as_str(&message.author.id, &message.id))
                        {
                            error!(target: "dungeon-helper", "Error sending message: Message ID: {}, Error: {:?}", message.id, why);
                        }
                    }
                }
            )
        }
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
