#[macro_use]
extern crate lazy_static;
extern crate log;

mod character;
mod character_roll;
mod command;
mod event_handler;
mod intent_logger;
mod intent_parser;
mod roll;

use crate::event_handler::Handler;
use log::error;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serenity::prelude::Client;
use snips_nlu_lib::SnipsNluEngine;
use std::env;

fn main() {
    env_logger::init();

    let bot_id = env::var("DISCORD_BOT_ID").expect("Expected a bot ID in the environment");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let engine = SnipsNluEngine::from_path("model").unwrap();

    let manager = SqliteConnectionManager::file("dungeon-helper.db");

    let pool = Pool::new(manager).expect("Error creating connection pool");

    let handler = Handler {
        bot_id,
        engine,
        pool,
    };

    let mut client = Client::new(&token, handler).expect("Error creating Discord client");

    if let Err(why) = client.start() {
        error!(target: "dungeon-helper", "Client error: {:?}", why);
    }
}
