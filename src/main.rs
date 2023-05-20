pub mod database;
pub mod models;
pub mod schema;

pub mod commands;
pub mod framework;

pub mod state;
pub use state::State;

// Use is here for ease of use through `crate::serenity`.
use poise::serenity_prelude as serenity;

use serde::Deserialize;
use std::fs;

// Our custom types for errors and contexts.
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, State, Error>;

#[derive(Deserialize)]
pub struct SelfRoleOptions {
    pub enabled: bool,
    pub channel: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub database_url: String,
    pub self_roles: SelfRoleOptions,
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
