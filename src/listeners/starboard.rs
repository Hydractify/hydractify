use poise::serenity_prelude::{Context, Mentionable, Reaction, ReactionType, User};

use crate::{models::Starboard, Error, State};

/// Collects the user IDs without duplicates
pub fn collect_users(reaction_users: Vec<User>) -> Vec<u64> {
    let mut users: Vec<u64> = reaction_users
        .iter()
        .map(|user| user.id.as_u64().clone())
        .collect();

    users.dedup();

    users
}

/// Handles the starboard when a reaction gets added.
pub async fn handle_reaction(
    ctx: &Context,
    state: &State,
    reaction: &Reaction,
    addition: bool,
) -> Result<(), Error> {
    if reaction.user(&ctx.http).await?.bot {
        return Ok(());
    }

    let message = reaction.message(&ctx.http).await?;

    // if message.author.id == reaction.user(&ctx.http).await?.id {
    //     return Ok(());
    // }

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

    for reac in message.reactions.iter().filter(|r| {
        config
            .emojis
            .iter()
            .any(|e| e == &r.reaction_type.to_string())
    }) {
        user_reactions = collect_users(
            reaction
                .users::<ReactionType, User>(&ctx.http, reac.reaction_type.clone(), Some(100), None)
                .await?,
        )
    }

    // Gotta create the variable here so we can await, you can't do that in a match.
    let message_link = message.link_ensured(&ctx.http).await;

    let mut entry = Starboard {
        message_id: i64::from(reaction.message_id),
        starboard_id: None,
        stars: user_reactions.len() as i32,
    };

    // Check wheter they're reacting to a starboard message, that way we don't create a new entry
    // but we also allow them to add stars into that starboard message.
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
                        channel.mention()
                    ))
                })
                .await?;

            entry.stars = db_entry.stars + if addition { 1 } else { -1 }
        }
        Err(_) => {
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
                        channel.mention()
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
                            if message.attachments.len() == 0 {
                                ""
                            } else {
                                &message.attachments.first().unwrap().url
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
