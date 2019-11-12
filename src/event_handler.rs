use crate::character::{
    Ability, Character, CharacterAttribute, CharacterAttributeUpdate, SavingThrow, Skill,
};
use crate::character_roll::CharacterRoll;
use crate::command::Command;
use crate::intent_parser::parse_intent_result;
use crate::roll::Roll;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use regex::Regex;
use snips_nlu_lib::SnipsNluEngine;

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

const CONNECTION_ERROR_TEXT: &str =
    "An unknown error occurred with obtaining a database connection.";

enum Response {
    Clarification(String),
    DiceRoll(String),
    Error(String),
    Help(String),
    Show(String),
    Update(String),
    Warning(String),
}

impl Response {
    pub fn as_str(&self, author_id: &UserId) -> String {
        match self {
            Response::Clarification(message) => format!("📎 <@{}> {}", author_id, message),
            Response::DiceRoll(message) => format!("🎲 <@{}> {}", author_id, message),
            Response::Error(message) => format!("💥 <@{}> **Error:** {}", author_id, message),
            Response::Help(message) => format!("🎱 <@{}> {}", author_id, message),
            Response::Show(message) => format!("<@{}> {}", author_id, message),
            Response::Update(message) => format!("💾 <@{}> {}", author_id, message),
            Response::Warning(message) => format!("⚠️ <@{}> {}", author_id, message),
        }
    }
}

pub struct Handler {
    pub bot_id: String,
    pub engine: SnipsNluEngine,
    pub pool: Pool<SqliteConnectionManager>,
}

impl Handler {
    fn get_response(&self, msg: &Message) -> Option<Response> {
        let author_id = &msg.author.id;
        let channel_id = &msg.channel_id;
        let content = &msg.content.trim();

        let command = Command::parse_shorthand(content).or(self.parse_message(content));

        command.map(|command| match command {
            Command::CharacterRoll(roll) => {
                self.get_character_roll_response(&roll, channel_id, author_id)
            }
            Command::Clarification(message) => Response::Clarification(message),
            Command::Error(message) => Response::Error(message),
            Command::Help => self.get_help_response(),
            Command::HelpShorthand => self.get_help_shorthand_response(),
            Command::Roll(roll) => Handler::get_roll_response(roll),
            Command::Set(attribute) => self.get_set_response(&attribute, channel_id, author_id),
            Command::Show(attribute) => self.get_show_response(&attribute, channel_id, author_id),
            Command::ShowAbilities => self.get_show_abilities_response(channel_id, author_id),
            Command::ShowSkills => self.get_show_skills_response(channel_id, author_id),
        })
    }

    fn extract_at_message(&self, message: &str) -> Option<String> {
        lazy_static! {
            static ref COMMAND_REGEX: Regex = Regex::new(r"^<@(\d+)> +(.*)$").unwrap();
        }

        COMMAND_REGEX.captures(&message).and_then(|c| {
            c.get(1)
                .filter(|m| m.as_str() == self.bot_id)
                .and(c.get(2).map(|m| m.as_str().to_string()))
        })
    }

    fn parse_message(&self, message: &str) -> Option<Command> {
        self.extract_at_message(message).map(|at_message| {
            let result = self.engine.parse(at_message.trim(), None, None);
            result
                .map(|result| {
                    parse_intent_result(result).unwrap_or(Command::Clarification("I'm not sure what you mean. Try asking again with a different or simpler phrasing. Try asking for help to see some examples.".to_string()))
                })
                .unwrap_or(Command::Error("An unknown error has occurred with understanding your message. Try again.".to_string()))
        })
    }

    fn get_character_roll_response(
        &self,
        character_roll: &CharacterRoll,
        channel_id: &ChannelId,
        author_id: &UserId,
    ) -> Response {
        self.pool
            .get()
            .map_err(|_| Response::Error(CONNECTION_ERROR_TEXT.to_string()))
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
            .unwrap_or_else(|error| error)
    }

    fn get_help_response(&self) -> Response {
        Response::Help(format!(
            "Try sending the following to <@{}>:\n\
             • \"Roll three d8s\"\n\
             • \"Throw two twelve-sided dice\"\n\
             • \"Do a strength check with advantage\"\n\
             • \"Perform a wisdom saving throw\"\n\
             • \"Try a stealth roll with disadvantage\"\n\
             • \"Roll for initiative\"\n\
             There are also short-hand commands you can use. Type \"!help\" for more info.",
            self.bot_id
        ))
    }

    fn get_help_shorthand_response(&self) -> Response {
        Response::Help(format!(
            "Try typing the following:\n\
             • \"!r 3d8\"\n\
             • \"!r 2d12+3\"\n\
             • \"!r strength with advantage\"\n\
             • \"!r wisdom saving throw\"\n\
             • \"!r stealth with disadvantage\"\n\
             • \"!r initiative\"\n\
             There are also natural language commands you can use. Type \"<@{}> help\" for more info.",
            self.bot_id
        ))
    }

    fn get_roll_response(roll: Roll) -> Response {
        let mut rng = rand::thread_rng();
        let result = roll.roll(&mut rng);
        Response::DiceRoll(format!("rolled {} = {}", roll, result))
    }

    fn get_set_response(
        &self,
        attribute: &CharacterAttributeUpdate,
        channel_id: &ChannelId,
        author_id: &UserId,
    ) -> Response {
        self.pool
            .get()
            .map_err(|_| Response::Error(CONNECTION_ERROR_TEXT.to_string()))
            .and_then(|mut connection| {
                Character::set_attribute(&mut connection, channel_id, author_id, attribute).map_err(
                    |_| {
                        Response::Error(
                            "An unknown error occurred with updating your character.".to_string(),
                        )
                    },
                )
            })
            .map(|_| Response::Update(format!("Set {}", attribute)))
            .unwrap_or_else(|error| error)
    }

    fn get_show_response(
        &self,
        attribute: &CharacterAttribute,
        channel_id: &ChannelId,
        author_id: &UserId,
    ) -> Response {
        self.pool
            .get()
            .map_err(|_| Response::Error(CONNECTION_ERROR_TEXT.to_string()))
            .and_then(|connection| {
                Character::get(&connection, channel_id, author_id)
                    .map_err(|_| Response::Warning(CHARACTER_NOT_FOUND_WARNING_TEXT.to_string()))
            })
            .map(|character| Response::Show(Handler::show_attribute(&character, attribute)))
            .unwrap_or_else(|error| error)
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

    fn get_show_abilities_response(&self, channel_id: &ChannelId, author_id: &UserId) -> Response {
        self.pool
            .get()
            .map_err(|_| Response::Error(CONNECTION_ERROR_TEXT.to_string()))
            .and_then(|connection| {
                Character::get(&connection, channel_id, author_id)
                    .map_err(|_| Response::Error("Error retrieving character.".to_string()))
            })
            .map(|character| {
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
            .unwrap_or_else(|error| error)
    }

    fn get_show_skills_response(&self, channel_id: &ChannelId, author_id: &UserId) -> Response {
        self.pool
            .get()
            .map_err(|_| Response::Error(CONNECTION_ERROR_TEXT.to_string()))
            .and_then(|connection| {
                Character::get(&connection, channel_id, author_id)
                    .map_err(|_| Response::Warning("Error retrieving character.".to_string()))
            })
            .map(|character| {
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
            .unwrap_or_else(|error| error)
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
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        let response = self.get_response(&msg);

        response.iter().for_each(|response| {
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, response.as_str(&msg.author.id))
            {
                println!("Error sending message: {:?}", why);
            }
        })
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
