use poise::serenity_prelude::{Context, Reaction};

use crate::{models::Starboard, Error, State};

pub async fn update_starboard(
    ctx: &Context,
    state: &State,
    reaction: &Reaction,
) -> Result<(), Error> {
    let config = &state.config.starboard;

    if !config.enabled {
        return Ok(());
    }

    if !config
        .emojis
        .iter()
        .any(|e| e == &reaction.emoji.to_string())
    {
        return Ok(());
    }

    let message = reaction.message(&ctx.http).await?;

    let reaction_count: u64;
    match message
        .reactions
        .iter()
        .find(|r| r.reaction_type.to_string() == reaction.emoji.to_string())
    {
        Some(reactions) => reaction_count = reactions.count,
        None => reaction_count = 0,
    }

    let message_link = message.link_ensured(&ctx.http).await;

    let mut entry = Starboard {
        message_id: i64::from(reaction.message_id),
        starboard_id: None,
        stars: reaction_count as i32,
    };

    match Starboard::find_one(
        &mut state.database_connection.get()?,
        i64::from(reaction.message_id),
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

            if reaction_count < config.threshold as u64 {
                starboard_message.delete(&ctx.http).await?;
                Starboard::delete_one(&mut state.database_connection.get()?, db_entry.message_id)?;

                return Ok(());
            }

            starboard_message
                .edit(&ctx.http, |msg| {
                    msg.content(format!("**{}**🌟『<#{}>』", reaction_count, channel.id().0))
                })
                .await?;

            entry.starboard_id = db_entry.starboard_id;
        }
        Err(_) => {
            if reaction_count < config.threshold as u64 {
                return Ok(());
            }

            let channel = ctx
                .http
                .get_channel(state.config.starboard.channel.parse::<u64>().unwrap())
                .await?;

            let msg_sent = channel
                .id()
                .send_message(&ctx.http, |msg| {
                    msg.content(format!("**{}**🌟『<#{}>』", reaction_count, channel.id().0))
                        .add_embed(|e| {
                            e.author(|a| {
                                a.name(message.author.tag())
                                    .icon_url(message.author.avatar_url().unwrap())
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

/// Handles the starboard when a reaction gets added.
pub async fn handle_reaction(
    ctx: &Context,
    state: &State,
    reaction: &Reaction,
) -> Result<(), Error> {
    if reaction.user(&ctx.http).await?.bot {
        return Ok(());
    }

    let message = reaction.message(&ctx.http).await?;

    if message.author.id == reaction.user(&ctx.http).await?.id {
        return Ok(());
    }

    update_starboard(ctx, state, reaction).await?;

    Ok(())
}
