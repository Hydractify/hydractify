use poise::serenity_prelude::{
    ComponentType, Context, InteractionResponseType, MessageComponentInteraction, RoleId,
};
use regex::Regex;

use crate::{Error, State};

pub async fn handle_interaction(
    ctx: &Context,
    state: &State,
    component: &MessageComponentInteraction,
) -> Result<(), Error> {
    let data = &component.data;

    // SelfRole module handling
    // If it is not enable, doesn't matches the regex below or
    // the component isn't a button, we ignore it.
    let regex = Regex::new(r"^self-role-(\d+)").unwrap();
    if !state.config.self_roles.enabled
        && !regex.is_match(&data.custom_id)
        && data.component_type != ComponentType::Button
    {
        return Ok(());
    }

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
                        msg.content("This role does not exists anymore, warn staff about it!")
                            .ephemeral(true)
                    })
            })
            .await?;

        return Ok(());
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
                        msg.content(format!("Successfully added <@&{}> to you.", button_role.0))
                            .ephemeral(true)
                    })
            })
            .await?;
    }

    Ok(())
}
