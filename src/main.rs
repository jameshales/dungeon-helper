#![feature(option_result_contains)]
#[macro_use]
extern crate lazy_static;

mod roll;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use regex::Regex;
use roll::Roll;
use std::env;

use serenity::{
    model::{channel::Message, gateway::Ready, id::{ChannelId, UserId}},
    prelude::*,
};

enum Command {
    Error { message: String },
    Help,
    Increment,
    Roll { roll: Roll },
}

impl Command {
    fn parse(command: &str) -> Option<Command> {
        lazy_static! {
            static ref ROLL_COMMAND_REGEX: Regex = Regex::new(r"^!(?:r|roll) +(.*)$").unwrap();
        }

        if command == "!help" {
            Some(Command::Help)
        } else if command == "!increment" {
            Some(Command::Increment)
        } else if let Some(captures) = ROLL_COMMAND_REGEX.captures(&command) {
            let roll_command = captures.get(1).map_or("", |m| m.as_str()).to_string();
            Some(Roll::parse(&roll_command)
                .map(|roll| Command::Roll { roll })
                .unwrap_or_else(|error| Command::Error { message: error.message().to_string() }))
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

        Command::parse(&msg.content).map(|command|
            match command {
                Command::Error { message } => Handler::get_error_response(&message, author_id),
                Command::Help => Handler::get_help_response(author_id),
                Command::Increment => self.get_increment_response(&msg.channel_id, author_id),
                Command::Roll { roll } => Handler::get_roll_response(roll, author_id),
            }
        )
    }

    fn get_error_response(message: &str, author_id: &UserId) -> String {
        format!(
            "<@{}> **Error:** {} Type `!help` for help.",
            author_id,
            message
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

    let handler = Handler {
        pool: pool
    };

    let mut client = Client::new(&token, handler).expect("Error creating Discord client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
