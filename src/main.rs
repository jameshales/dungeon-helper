#![feature(option_result_contains)]
#[macro_use]
extern crate lazy_static;

mod roll;

use regex::Regex;
use roll::Roll;
use std::env;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        lazy_static! {
            static ref ROLL_COMMAND_REGEX: Regex = Regex::new(r"^!(?:r|roll) +(.*)$").unwrap();
        }

        let response = if msg.content == "!help" {
            Some(format!("<@{}> **Usage:** `!roll [n]d[n] [ [+|-] n] [with [advantage|disadvantage]]`.\n**Examples:** `!roll 1d20`, `!roll 2d8 + 3`, `!roll 3d4 - 2 with advantage`.", msg.author.id))
        } else if let Some(captures) = ROLL_COMMAND_REGEX.captures(&msg.content) {
            let mut rng = rand::thread_rng();

            let roll_command = captures.get(1).map_or("", |m| m.as_str());

            Some(match Roll::parse(&roll_command) {
                Ok(roll) => {
                    let result = roll.roll(&mut rng);
                    format!("🎲 <@{}> rolled {} = {}", msg.author.id, roll, result)
                }
                Err(e) => format!(
                    "<@{}> **Error:** {} Type `!help` for help.",
                    msg.author.id,
                    e.message()
                ),
            })
        } else {
            None
        };

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

    let mut client = Client::new(&token, Handler).expect("Err creating client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
