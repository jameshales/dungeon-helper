#[macro_use]
extern crate lazy_static;

mod character;
mod character_roll;
mod command;
mod event_handler;
mod roll;

use crate::event_handler::Handler;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serenity::prelude::Client;
use std::env;

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
