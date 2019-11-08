#[macro_use]
extern crate lazy_static;

mod character;
mod character_roll;
mod roll;

use character::{Ability, Character, CharacterAttribute};
use character_roll::CharacterRoll;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use regex::Regex;
use roll::Roll;
use std::convert::identity;
use std::env;

use serenity::{
    model::{
        channel::Message,
        gateway::Ready,
        id::{ChannelId, UserId},
    },
    prelude::*,
};

enum Command {
    Error { message: String },
    CharacterRoll { roll: CharacterRoll },
    Help,
    Increment,
    Roll { roll: Roll },
    Set { attribute: CharacterAttribute },
    ShowAbilities,
}

impl Command {
    fn parse(command: &str) -> Option<Command> {
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

struct Handler {
    pool: Pool<SqliteConnectionManager>,
}

impl Handler {
    fn get_response(&self, msg: &Message) -> Option<String> {
        let author_id = &msg.author.id;
        let channel_id = &msg.channel_id;

        Command::parse(&msg.content).map(|command| match command {
            Command::Error { message } => Handler::get_error_response(&message, author_id),
            Command::Help => Handler::get_help_response(author_id),
            Command::Increment => self.get_increment_response(channel_id, author_id),
            Command::Roll { roll } => Handler::get_roll_response(roll, author_id),
            Command::Set { attribute } => self.get_set_response(&attribute, channel_id, author_id),
            Command::CharacterRoll { roll } => self.get_character_roll_response(&roll, channel_id, author_id),
            Command::ShowAbilities => self.get_show_abilities_response(channel_id, author_id),
        })
    }

    fn get_character_roll_response(&self, character_roll: &CharacterRoll, channel_id: &ChannelId, author_id: &UserId) -> String {
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
            .and_then(|character|
                character_roll.to_roll(&character).ok_or(format!("<@{}> Missing stats in character.", author_id))
            ).map(|roll| {
                let mut rng = rand::thread_rng();
                let result = roll.roll(&mut rng);
                format!("🎲 <@{}> rolled {} ({}) = {}", author_id, character_roll.check, roll, result)
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

    fn get_increment_response(&self, channel_id: &ChannelId, author_id: &UserId) -> String {
        self.pool.get()
            .map_err(|_| format!("<@{}> Error obtaining connection from connection pool.", author_id))
            .and_then(|connection|
                connection.execute(
                    "INSERT INTO counters (channel_id, user_id, counter) VALUES ($1, $2, 0) ON CONFLICT (channel_id, user_id) DO UPDATE SET counter = counter + 1 WHERE channel_id = $1 AND user_id = $2",
                    &[
                        &channel_id.to_string(),
                        &author_id.to_string()
                    ]
                )
                .map_err(|_| format!("<@{}> Error incrementing counter.", author_id))
                .and(
                    connection.query_row(
                        "SELECT counter FROM counters WHERE channel_id = $1 AND user_id = $2",
                        &[
                            &channel_id.to_string(),
                            &author_id.to_string()
                        ],
                        |row| {
                            row.get::<_, i32>(0)
                        }
                    )
                    .map_err(|_| format!("<@{}> Error retrieving updated counter.", author_id))
                )
            )
            .map(|counter| format!("<@{}> incremented counter to {}", author_id, counter))
            .unwrap_or_else(|error| error)
    }

    fn get_roll_response(roll: Roll, author_id: &UserId) -> String {
        let mut rng = rand::thread_rng();
        let result = roll.roll(&mut rng);
        format!("🎲 <@{}> rolled {} = {}", author_id, roll, result)
    }

    fn get_set_response(&self, attribute: &CharacterAttribute, channel_id: &ChannelId, author_id: &UserId) -> String {
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
            .map(|_| {
                format!(
                    "<@{}> Updated character successfully.",
                    author_id,
                )
            })
            .unwrap_or_else(|error| error)
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
                    "<@{}> STR = {}, DEX = {}, CON = {}, INT = {}, WIS = {}, CHA = {}",
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

    fn format_ability(ability: Option<Ability>) -> String {
        ability.map_or("?".to_string(), |a| {
            format!("{:+} ({})", a.modifier, a.score)
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

fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let manager = SqliteConnectionManager::file("dungeon-helper.db");

    let pool = Pool::new(manager).expect("Error creating connection pool");

    let handler = Handler { pool: pool };

    let mut client = Client::new(&token, handler).expect("Error creating Discord client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
