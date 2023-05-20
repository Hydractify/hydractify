use poise::serenity_prelude::{
    self as serenity, ComponentType, Interaction, InteractionResponseType, RoleId,
};
use regex::Regex;

use super::commands;
use crate::{Config, Error, State};

// Handle Discord events
// https://discord.com/developers/docs/topics/gateway-events#receive-events
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
                let data = &component.data;

                // SelfRole module handling
                // If it is not enable, doesn't matches the regex below or
                // the component isn't a button, we ignore it.
                let regex = Regex::new(r"^self-role-(\d+)").unwrap();
                if state.config.self_roles.enabled
                    && regex.is_match(&data.custom_id)
                    && data.component_type == ComponentType::Button
                {
                    let guild_roles = component.guild_id.unwrap().roles(&ctx.http).await?;
                    let button_role = RoleId::from(
                        regex
                            .captures(&data.custom_id)
                            .unwrap()
                            .get(1)
                            .unwrap()
                            .as_str()
                            .parse::<u64>()?,
                    );

                    // If the guild doesn't contains the role we wanna add/remove, warn user.
                    if !guild_roles.contains_key(&button_role) {
                        component
                            .create_interaction_response(&ctx.http, |resp| {
                                resp.kind(InteractionResponseType::ChannelMessageWithSource)
                                    .interaction_response_data(|msg| {
                                        msg.content("This role does not exists anymore, warn staff about it!").ephemeral(true)
                                    })
                            })
                            .await?;

                        ()
                    }

                    let mut member = component.member.clone().unwrap();

                    // If the user has the role already, we remove it, otherwise we add it.
                    if member.roles.iter().any(|r| r.0 == button_role.0) {
                        member.remove_role(&ctx.http, button_role).await?;
                        component
                            .create_interaction_response(&ctx.http, |resp| {
                                resp.kind(InteractionResponseType::ChannelMessageWithSource)
                                    .interaction_response_data(|msg| {
                                        msg.content(format!(
                                            "Successfully removed <@&{}> from you.",
                                            button_role.0
                                        ))
                                        .ephemeral(true)
                                    })
                            })
                            .await?;
                    } else {
                        member.add_role(&ctx.http, button_role).await?;
                        component
                            .create_interaction_response(&ctx.http, |resp| {
                                resp.kind(InteractionResponseType::ChannelMessageWithSource)
                                    .interaction_response_data(|msg| {
                                        msg.content(format!(
                                            "Successfully added <@&{}> to you.",
                                            button_role.0
                                        ))
                                        .ephemeral(true)
                                    })
                            })
                            .await?;
                    }
                }
            }

            Ok(())
        }
        _ => Ok(()),
    }
}

// Configures and starts the Discord application
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
                | serenity::GatewayIntents::GUILD_MESSAGE_REACTIONS, // Not needed at the moment,
                                                                     // but when we implement the starboard module, it will be.
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
