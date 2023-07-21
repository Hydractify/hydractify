use std::collections::HashMap;

use poise::serenity_prelude::{
    model::application::component::ButtonStyle, ChannelId, CreateButton, CreateComponents,
    Mentionable, ReactionType, Role, RoleId,
};

use crate::{models::SelfRole, Context, Error};

#[derive(poise::ChoiceParameter)]
#[repr(i32)]
pub enum StyleOptions {
    #[name = "Primary (Blurple)"]
    Primary = ButtonStyle::Primary as i32,
    #[name = "Secondary (Grey)"]
    Secondary = ButtonStyle::Secondary as i32,
    #[name = "Success (Green)"]
    Success = ButtonStyle::Success as i32,
    #[name = "Danger (Red)"]
    Danger = ButtonStyle::Danger as i32,
}

impl StyleOptions {
    // Allows us to convert a `StyleOptions` value into `ButtonStyle`.
    pub fn convert_from_int(value: i32) -> ButtonStyle {
        match value {
            x if x == StyleOptions::Secondary as i32 => ButtonStyle::Secondary,
            x if x == StyleOptions::Success as i32 => ButtonStyle::Success,
            x if x == StyleOptions::Danger as i32 => ButtonStyle::Danger,
            _ => ButtonStyle::Primary,
        }
    }

    pub fn name_from_int(value: i32) -> &'static str {
        match value {
            x if x == StyleOptions::Primary as i32 => "PRIMARY",
            x if x == StyleOptions::Secondary as i32 => "SECONDARY",
            x if x == StyleOptions::Success as i32 => "SUCCESS",
            x if x == StyleOptions::Danger as i32 => "DANGER",
            _ => "UNKNOWN",
        }
    }
}

/// Creates the common component that we send when deploying self roles.
pub fn create_component<'a>(
    menu: &'a mut CreateComponents,
    roles: HashMap<RoleId, (&Role, SelfRole)>,
) -> &'a mut CreateComponents {
    menu.create_action_row(|row| {
        for role in roles {
            let guild_role = role.1 .0;
            let database_data = role.1 .1;

            let mut button = CreateButton::default();
            button
                .label(&guild_role.name)
                .style(StyleOptions::convert_from_int(database_data.style))
                .custom_id(format!("self-role-{}", database_data.id));

            if let Some(emoji) = database_data.emoji {
                button.emoji(emoji.parse::<ReactionType>().unwrap());
            }

            row.add_button(button);
        }

        row
    })
}

#[poise::command(
    slash_command,
    rename = "selfrole",
    subcommands("add", "remove", "deploy", "show", "cleanup")
)]
pub async fn self_role(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

// Perhaps we should eventually use selection menus instead of buttons? That way users can only add
// what hasn't already been added.
/// Add a role to the list of available self roles.
#[poise::command(slash_command, ephemeral)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "The role to add to the list of self roles"] role: Role,
    #[description = "The button style to use"] style: StyleOptions,
    #[description = "The emoji to use. (Make sure its actually valid)"] emoji: String,
) -> Result<(), Error> {
    if role.id.as_u64() == ctx.guild().unwrap().id.as_u64() {
        ctx.send(|message| {
            message.content(
                "The everyone role may not be self-assignable; It's the everyone role after all.",
            )
        })
        .await?;

        return Ok(());
    }

    let database_connection = &mut ctx.data().database_connection.get()?;

    // Stores the self role for later checking if there was one already registered or not.
    let selfrole_result = SelfRole::find_one(database_connection, i64::from(role.id));
    SelfRole::upsert(
        database_connection,
        SelfRole {
            id: i64::from(role.id),
            style: style as i32,
            emoji: Some(emoji),
        },
    )?;

    ctx.send(|message| {
        if let Err(_) = selfrole_result {
            message.content(format!(
                concat!(
                    "The <@&{}> role has been added to the list!\n",
                    "Make sure to `/selfrole deploy` this change."
                ),
                role.id
            ))
        } else {
            message.content(format!(
                concat!(
                    "Update style and emoji for the <@&{}> role.\n",
                    "Make sure to `/selfrole deploy` this change."
                ),
                role.id
            ))
        }
    })
    .await?;

    Ok(())
}

/// Remove a role from the list of available self roles.
#[poise::command(slash_command, ephemeral)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "The role to remove from the list of self roles"] role: Role,
) -> Result<(), Error> {
    let removed = SelfRole::remove(
        &mut ctx.data().database_connection.get()?,
        i64::from(role.id),
    )?;

    let response = if removed {
        format!(
            concat!(
                "The {} role is no longer self assignable!\n",
                "Make sure to `/selfrole deploy` this change."
            ),
            role
        )
    } else {
        format!("The {role} role is not self assignable.")
    };

    ctx.say(response).await?;

    Ok(())
}

/// Deploys the previously configured self roles.
///
/// If you have not configured it yet, do so through `/selfrole add`!
#[poise::command(slash_command, ephemeral)]
pub async fn deploy(
    ctx: Context<'_>,
    #[description = "The id of the message in the self roles channel to update."] message: Option<
        String,
    >,
) -> Result<(), Error> {
    let self_roles = SelfRole::get_all(&mut ctx.data().database_connection.get()?)?;

    let guild = ctx.guild().unwrap();

    // Creates a `HasMap` containing the database's self role data, and the Guild's role data.
    let mut guild_roles = HashMap::new();
    for role in self_roles {
        if let Some(r) = guild.roles.get(&RoleId::from(role.id as u64)) {
            guild_roles.insert(r.id, (r, role));
        }
    }

    let guild_channels = guild.channels(&ctx.serenity_context().http).await?;
    let channel = guild_channels
        .get(&ChannelId::from(
            ctx.data().config.self_roles.channel.parse::<u64>().unwrap(),
        ))
        .unwrap();

    // The menu creation needs to be ported into a macro or function... It's just very annoying to
    // do so, but maybe I'm retarded, it's 5AM after all.
    match message {
        // Checks whether the message provided actually exists.
        Some(msg) => {
            if let Ok(mut m) = channel
                .message(
                    &ctx.serenity_context().http,
                    u64::from(msg.parse::<u64>().unwrap()),
                )
                .await
            {
                m.edit(&ctx.serenity_context().http, |old_message| {
                    old_message
                        .content("Click the buttons below to toggle a role on you.")
                        .components(|menu| create_component(menu, guild_roles))
                })
                .await?;

                ctx.send(|msg| {
                    msg.content(format!(
                        concat!(
                            "Successfully sent a message for self roles into <#{}>",
                            "You may want to delete the old message if there is one, now."
                        ),
                        channel.id
                    ))
                })
                .await?;
            }
        }
        None => {
            channel
                .send_message(&ctx.serenity_context().http, |msg| {
                    msg.content("Click the buttons below to toggle a role on you.")
                        .components(|menu| create_component(menu, guild_roles))
                })
                .await?;

            ctx.send(|msg| {
                msg.content(format!(
                    concat!(
                        "Successfully edited the message for self roles in <#{}>",
                        "You may want to delete the old message if there is one, now."
                    ),
                    channel.id
                ))
            })
            .await?;
        }
    }

    Ok(())
}

/// Show all available self roles.
#[poise::command(slash_command, ephemeral)]
pub async fn show(ctx: Context<'_>) -> Result<(), Error> {
    let roles = SelfRole::get_all(&mut ctx.data().database_connection.get()?)?
        .iter()
        .map(|self_role| {
            format!(
                "{} (emoji: {}, style: {})",
                RoleId::from(self_role.id as u64).mention(),
                self_role.emoji.as_ref().unwrap_or(&String::from("none")),
                StyleOptions::name_from_int(self_role.style)
            )
        })
        .collect::<Vec<String>>()
        .join(", ");

    let response = if roles.is_empty() {
        "There are not self assignable roles.".into()
    } else {
        format!("Self assignable roles are: {}", roles)
    };

    ctx.say(response).await?;

    Ok(())
}

/// Removes deleted roles from the list of self assignable roles.
#[poise::command(slash_command, ephemeral)]
pub async fn cleanup(ctx: Context<'_>) -> Result<(), Error> {
    let connection = &mut ctx.data().database_connection.get()?;

    let guild_roles = ctx.guild().unwrap().roles;

    let deleted_roles: Vec<i64> = SelfRole::get_all(connection)?
        .iter()
        .filter_map(|self_role| {
            if guild_roles.contains_key(&RoleId::from(self_role.id as u64)) {
                Some(self_role.id)
            } else {
                None
            }
        })
        .collect();

    let response = if deleted_roles.is_empty() {
        "There are no roles to prune.".into()
    } else {
        SelfRole::remove_all(connection, &deleted_roles)?;

        format!(
            "Removed {} no longer existing self assignable roles from the list.",
            deleted_roles.len()
        )
    };

    ctx.say(response).await?;

    Ok(())
}
