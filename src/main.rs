pub mod database;
pub mod models;
pub mod schema;

pub mod commands;
pub mod framework;
pub mod listeners;

pub mod state;
pub use state::State;

use serde::Deserialize;
use std::fs;

// Our custom types for errors and contexts.
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, State, Error>;

#[derive(Deserialize)]
pub struct SelfRoleOptions {
    pub channel: String,
    pub enabled: bool,
}

#[derive(Deserialize)]
pub struct StarboardOptions {
    pub channel: String,
    pub emojis: Vec<String>,
    pub enabled: bool,
    pub threshold: u8,
}

#[derive(Deserialize)]
pub struct Config {
    pub self_roles: SelfRoleOptions,
    pub starboard: StarboardOptions,
    pub token: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Reads configuration to pass through the bot.
    let config: Config = toml::from_str(&fs::read_to_string("./config.toml").unwrap()).unwrap();

    // Starts up the bot.
    framework::start(config).await?;

    Ok(())
}
