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

pub struct Handler {
    pub bot_id: String,
    pub engine: SnipsNluEngine,
    pub pool: Pool<SqliteConnectionManager>,
}

impl Handler {
    fn get_response(&self, msg: &Message) -> Option<String> {
        let author_id = &msg.author.id;
        let channel_id = &msg.channel_id;

        let command = Command::parse_roll(&msg.content).or(self.parse_message(&msg.content));

        command.map(|command| match command {
            Command::CharacterRoll(roll) => {
                self.get_character_roll_response(&roll, channel_id, author_id)
            }
            Command::Error(message) => Handler::get_error_response(&message, author_id),
            Command::Help => Handler::get_help_response(author_id),
            Command::Roll(roll) => Handler::get_roll_response(roll, author_id),
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
                    parse_intent_result(result).unwrap_or(Command::Error("No intent".to_string()))
                })
                .unwrap_or(Command::Error("Error parsing intent".to_string()))
        })
    }

    fn get_character_roll_response(
        &self,
        character_roll: &CharacterRoll,
        channel_id: &ChannelId,
        author_id: &UserId,
    ) -> String {
        self.pool
            .get()
            .map_err(|_| {
                format!(
                    "<@{}> Error obtaining connection from connection pool.",
                    author_id
                )
            })
            .and_then(|connection| {
                Character::get(&connection, channel_id, author_id)
                    .map_err(|_| format!("<@{}> Error retrieving character.", author_id))
            })
            .and_then(|character| {
                character_roll
                    .to_roll(&character)
                    .ok_or(format!("<@{}> Missing stats in character.", author_id))
            })
            .map(|roll| {
                let mut rng = rand::thread_rng();
                let result = roll.roll(&mut rng);
                format!(
                    "🎲 <@{}> rolled {} ({}) = {}",
                    author_id, character_roll.check, roll, result
                )
            })
            .unwrap_or_else(|error| error)
    }

    fn get_error_response(message: &str, author_id: &UserId) -> String {
        format!(
            "<@{}> **Error:** {} Type `!help` for help.",
            author_id, message
        )
    }

    fn get_help_response(author_id: &UserId) -> String {
        format!("<@{}> **Usage:** `!roll [n]d[n] [ [+|-] n] [with [advantage|disadvantage]]`.\n**Examples:** `!roll 1d20`, `!roll 2d8 + 3`, `!roll 3d4 - 2 with advantage`.", author_id)
    }

    fn get_roll_response(roll: Roll, author_id: &UserId) -> String {
        let mut rng = rand::thread_rng();
        let result = roll.roll(&mut rng);
        format!("🎲 <@{}> rolled {} = {}", author_id, roll, result)
    }

    fn get_set_response(
        &self,
        attribute: &CharacterAttributeUpdate,
        channel_id: &ChannelId,
        author_id: &UserId,
    ) -> String {
        self.pool
            .get()
            .map_err(|_| {
                format!(
                    "<@{}> Error obtaining connection from connection pool.",
                    author_id
                )
            })
            .and_then(|connection| {
                Character::set_attribute(&connection, channel_id, author_id, attribute)
                    .map_err(|_| format!("<@{}> Error updating character.", author_id))
            })
            .map(|_| format!("<@{}> Updated character successfully.", author_id,))
            .unwrap_or_else(|error| error)
    }

    fn get_show_response(
        &self,
        attribute: &CharacterAttribute,
        channel_id: &ChannelId,
        author_id: &UserId,
    ) -> String {
        self.pool
            .get()
            .map_err(|_| {
                format!(
                    "<@{}> Error obtaining connection from connection pool.",
                    author_id
                )
            })
            .and_then(|connection| {
                Character::get(&connection, channel_id, author_id)
                    .map_err(|_| format!("<@{}> Error retrieving character.", author_id))
            })
            .map(|character| {
                format!(
                    "<@{}> {}",
                    author_id,
                    Handler::show_attribute(&character, attribute)
                )
            })
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
                    "yes"
                } else {
                    "no"
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

    fn get_show_abilities_response(&self, channel_id: &ChannelId, author_id: &UserId) -> String {
        self.pool
            .get()
            .map_err(|_| {
                format!(
                    "<@{}> Error obtaining connection from connection pool.",
                    author_id
                )
            })
            .and_then(|connection| {
                Character::get(&connection, channel_id, author_id)
                    .map_err(|_| format!("<@{}> Error retrieving character.", author_id))
            })
            .map(|character| {
                format!(
                    "<@{}>\n\
                     STR = {}\n\
                     DEX = {}\n\
                     CON = {}\n\
                     INT = {}\n\
                     WIS = {}\n\
                     CHA = {}",
                    author_id,
                    Handler::format_ability(character.strength()),
                    Handler::format_ability(character.dexterity()),
                    Handler::format_ability(character.constitution()),
                    Handler::format_ability(character.intelligence()),
                    Handler::format_ability(character.wisdom()),
                    Handler::format_ability(character.charisma()),
                )
            })
            .unwrap_or_else(|error| error)
    }

    fn get_show_skills_response(&self, channel_id: &ChannelId, author_id: &UserId) -> String {
        self.pool
            .get()
            .map_err(|_| {
                format!(
                    "<@{}> Error obtaining connection from connection pool.",
                    author_id
                )
            })
            .and_then(|connection| {
                Character::get(&connection, channel_id, author_id)
                    .map_err(|_| format!("<@{}> Error retrieving character.", author_id))
            })
            .map(|character| {
                format!(
                    "<@{}>\n\
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
                    author_id,
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
                )
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
            if let Err(why) = msg.channel_id.say(&ctx.http, response) {
                println!("Error sending message: {:?}", why);
            }
        })
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
