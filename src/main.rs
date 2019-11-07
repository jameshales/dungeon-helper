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
    model::{channel::Message, gateway::Ready, user::User},
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
        Command::parse(&msg.content).map(|command|
            match command {
                Command::Error { message } => Handler::get_error_response(&message, &msg.author),
                Command::Help => Handler::get_help_response(&msg.author),
                Command::Increment => self.get_increment_response(&msg.author),
                Command::Roll { roll } => Handler::get_roll_response(roll, &msg.author),
            }
        )
    }

    fn get_error_response(message: &str, author: &User) -> String {
        format!(
            "<@{}> **Error:** {} Type `!help` for help.",
            author.id,
            message
        )
    }

    fn get_help_response(author: &User) -> String {
        format!("<@{}> **Usage:** `!roll [n]d[n] [ [+|-] n] [with [advantage|disadvantage]]`.\n**Examples:** `!roll 1d20`, `!roll 2d8 + 3`, `!roll 3d4 - 2 with advantage`.", author.id)
    }

    fn get_increment_response(&self, author: &User) -> String {
        self.pool.get()
            .map_err(|_| format!("<@{}> Error obtaining connection from connection pool.", author.id))
            .and_then(|connection|
                connection.execute(
                    "INSERT INTO counters (user_id, counter) VALUES ($1, 0) ON CONFLICT (user_id) DO UPDATE SET counter = counter + 1 WHERE user_id = $1",
                    &[&author.id.to_string()]
                )
                .map(|_| format!("<@{}> Incremented!", author.id))
                .map_err(|_| format!("<@{}> Error incrementing counter.", author.id))
            )
            .unwrap_or_else(|error| error)
    }

    fn get_roll_response(roll: Roll, author: &User) -> String {
        let mut rng = rand::thread_rng();
        let result = roll.roll(&mut rng);
        format!("🎲 <@{}> rolled {} = {}", author.id, roll, result)
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
