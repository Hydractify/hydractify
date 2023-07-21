use poise::serenity_prelude::{self as serenity, Interaction};

use super::commands;
use crate::{
    listeners::{self_role, starboard},
    Config, Error, State,
};

/// Handle Discord events
/// <https://discord.com/developers/docs/topics/gateway-events#receive-events>
async fn listener(
    ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _: poise::FrameworkContext<'_, State, Error>,
    state: &State,
) -> Result<(), Error> {
    match event {
        poise::Event::Ready { data_about_bot } => {
            println!("{} is ready.", data_about_bot.user.tag());

            Ok(())
        }
        poise::Event::InteractionCreate { interaction } => {
            if let Interaction::MessageComponent(component) = interaction {
                self_role::handle_interaction(ctx, state, component).await?;
            }

            Ok(())
        }
        poise::Event::ReactionAdd { add_reaction } => {
            starboard::handle_reaction(ctx, state, add_reaction, true).await
        }
        poise::Event::ReactionRemove { removed_reaction } => {
            starboard::handle_reaction(ctx, state, removed_reaction, false).await
        }
        _ => Ok(()),
    }
}

/// Configures and starts the Discord application
pub async fn start(config: Config) -> Result<(), Error> {
    poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::self_role()],
            event_handler: |ctx, event, framework, state| {
                Box::pin(listener(ctx, event, framework, state))
            },
            on_error: |error| {
                Box::pin(async move {
                    if let Err(err) = poise::builtins::on_error(error).await {
                        println!("Error while handling error: {}", err);
                    }
                })
            },
            ..Default::default()
        })
        .token(&config.token)
        .intents(
            serenity::GatewayIntents::non_privileged()
                | serenity::GatewayIntents::GUILDS
                | serenity::GatewayIntents::GUILD_MEMBERS
                | serenity::GatewayIntents::GUILD_MESSAGES
                | serenity::GatewayIntents::GUILD_MESSAGE_REACTIONS,
        )
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                // Register all the commands we have.
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                // Load global state into the framework, can be accessed through
                // <crate::Context>.data()
                Ok(State::load(ctx, config).await)
            })
        })
        .run()
        .await?;

    Ok(())
}
