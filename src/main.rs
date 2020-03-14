#[macro_use]
extern crate lazy_static;
extern crate log;
extern crate symspell;

mod attack_roll;
mod channel;
mod character;
mod character_roll;
mod command;
mod error;
mod event_handler;
mod intent_logger;
mod intent_parser;
mod response;
mod roll;
mod weapon;

use crate::event_handler::Handler;
use log::error;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serenity::prelude::Client;
use snips_nlu_lib::SnipsNluEngine;
use std::env;
use std::sync::RwLock;
use symspell::{SymSpell, UnicodeStringStrategy};

fn main() {
    env_logger::init();

    let database_path =
        env::var("DATABASE_PATH").expect("Expected a database path in the environment");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let model_path = env::var("MODEL_PATH").expect("Expected a model path in the environment");
    let dictionary_path = env::var("DICTIONARY_PATH").expect("Expected a dictionary path in the environment");
    let bigram_dictionary_path = env::var("BIGRAM_DICTIONARY_PATH").expect("Expected a bigram dictionary path in the environment");

    let engine = SnipsNluEngine::from_path(model_path).unwrap();

    let manager = SqliteConnectionManager::file(database_path);

    let pool = Pool::new(manager).expect("Error creating connection pool");

    let mut symspell: SymSpell<UnicodeStringStrategy> = SymSpell::default();

    symspell.load_dictionary(&dictionary_path, 0, 1, " ");
    symspell.load_bigram_dictionary(
      &bigram_dictionary_path,
      0,
      2,
      " "
    );

    let handler = Handler {
        bot_id: RwLock::new(None),
        engine,
        pool,
        symspell,
    };

    let mut client = Client::new(&token, handler).expect("Error creating Discord client");

    if let Err(why) = client.start() {
        error!(target: "dungeon-helper", "Client error: {:?}", why);
    }
}
