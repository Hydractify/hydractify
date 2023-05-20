use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

use crate::{database, serenity, Config};

// Definition of the bot's state, shared in all commands.
pub struct State {
    pub database_connection: Pool<ConnectionManager<PgConnection>>,
    pub config: crate::Config,
}

// Implementation of the state, so we actually load values into it.
impl State {
    pub async fn load(_ctx: &serenity::Context, config: Config) -> Self {
        Self {
            database_connection: database::establish_connection(&config.database_url),
            config,
        }
    }
}
