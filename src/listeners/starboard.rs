use poise::serenity_prelude::{Context, Mentionable, Reaction, ReactionType, User, UserId};

use crate::{models::Starboard, Error, State};

/// Collects the user IDs without duplicates
pub fn collect_users(reaction_users: Vec<User>, message_user_id: UserId) -> Vec<u64> {
    let mut users: Vec<u64> = reaction_users
        .iter()
        .filter_map(|user| {
            if user.bot {
                // Make sure we don't count reactions added by bots.
                None
            } else if message_user_id == user.id {
                // Make sure we also don't count a
                // reaction from the creator of the message.
                None
            } else {
                Some(user.id.as_u64().clone())
            }
        })
        .collect();

    // This will remove any IDs that repeat from the vector.
    users.dedup();

    users
}

/// Handles the starboard when a reaction gets added.
pub async fn handle_reaction(
    ctx: &Context,
    state: &State,
    reaction: &Reaction,
) -> Result<(), Error> {
    let user_from_reaction = reaction.user(&ctx.http).await?;

    // Stop if it's a bot who reacted.
    if user_from_reaction.bot {
        return Ok(());
    }

    let message = reaction.message(&ctx.http).await?;

    // Stop if the reaction was added by the user.
    if message.author.id == user_from_reaction.id {
        return Ok(());
    }

    // The configuration of the starboard, found in `config.toml`.
    let config = &state.config.starboard;

    // Make sure we don't handle the starboard if we disabled it.
    if !config.enabled {
        return Ok(());
    }

    // If none of the emotes we configured to trigger the starboard aren't the one that was just
    // added, we ignore.
    if !config
        .emojis
        .iter()
        .any(|e| e == &reaction.emoji.to_string())
    {
        return Ok(());
    }

    // Go through all the reactions and count the valid ones for starboard.
    let mut user_reactions: Vec<u64> = Vec::new();

    for reac in message.reactions.iter() {
        if config
            .emojis
            .iter()
            .any(|e| e == &reac.reaction_type.to_string())
        {
            user_reactions.append(&mut collect_users(
                reaction
                    .users::<ReactionType, User>(
                        &ctx.http,
                        reac.reaction_type.clone(),
                        Some(100),
                        None,
                    )
                    .await?,
                message.author.id, // We pass the author ID to filter their reactions out.
            ))
        }
    }
    user_reactions.dedup(); // Make sure we don't have duplicate users from multiple reactions.

    // Gotta create the variable here so we can await, you can't do that in a match.
    let message_link = message.link_ensured(&ctx.http).await;

    let mut entry = Starboard {
        message_id: i64::from(reaction.message_id),
        starboard_id: None,
        stars: user_reactions.len() as i32,
    };

    // Check wheter they're reacting to a starboard message, that way we don't create a new entry
    // but we also allow them to add stars into that starboard message.
    // TODO: This is super hacky, let's uh... Work on this at some point please. We fetch twice
    // because of this logic.
    let mut starboard_id = None;
    let mut message_id = None;
    if let Ok(_) = Starboard::find_one(
        &mut state.database_connection.get()?,
        None,
        Some(i64::from(message.id)),
    ) {
        starboard_id = Some(i64::from(message.id));
        println!("Found starboard");
    } else {
        message_id = Some(i64::from(reaction.message_id));
    }

    match Starboard::find_one(
        &mut state.database_connection.get()?,
        message_id,
        starboard_id,
    ) {
        Ok(db_entry) => {
            // If there is an entry, update the message in the starboard channel.
            let channel = ctx
                .http
                .get_channel(state.config.starboard.channel.parse::<u64>().unwrap())
                .await?;

            let mut starboard_message = channel
                .id()
                .message(&ctx.http, db_entry.starboard_id.unwrap() as u64)
                .await?;

            if user_reactions.len() < config.threshold as usize {
                starboard_message.delete(&ctx.http).await?;
                Starboard::delete_one(&mut state.database_connection.get()?, db_entry.message_id)?;

                return Ok(());
            }

            starboard_message
                .edit(&ctx.http, |msg| {
                    msg.content(format!(
                        "**{}**🌟『{}』",
                        user_reactions.len(),
                        message.channel_id.mention()
                    ))
                })
                .await?;
        }
        Err(_) => {
            // If there isn't an entry, create a message in the starboard channel.
            if user_reactions.len() < config.threshold as usize {
                return Ok(());
            }

            let channel = ctx
                .http
                .get_channel(state.config.starboard.channel.parse::<u64>().unwrap())
                .await?;

            let msg_sent = channel
                .id()
                .send_message(&ctx.http, |msg| {
                    msg.content(format!(
                        "**{}**🌟『{}』",
                        user_reactions.len(),
                        message.channel_id.mention()
                    ))
                    .add_embed(|e| {
                        e.author(|a| {
                            a.name(message.author.tag());

                            if let Some(avatar_url) = message.author.avatar_url() {
                                a.icon_url(avatar_url);
                            }

                            return a;
                        })
                        .field("Message", message.content, false)
                        .color(0xffcf05)
                        .description(format!("[Original]({})", message_link))
                        .image(format!(
                            "{}",
                            if message.attachments.len() != 0 {
                                &message.attachments.first().unwrap().url
                            } else if message.embeds.len() != 0 {
                                let embed = &message.embeds.first().unwrap();

                                if let Some(image) = &embed.image {
                                    &image.url
                                } else if let Some(image) = &embed.thumbnail {
                                    &image.url
                                } else {
                                    ""
                                }
                            } else {
                                ""
                            }
                        ))
                    })
                })
                .await?;

            entry.starboard_id = Some(i64::from(msg_sent.id));
        }
    }

    Starboard::upsert(&mut state.database_connection.get()?, entry)?;

    Ok(())
}
