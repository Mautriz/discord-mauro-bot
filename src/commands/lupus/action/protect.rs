use crate::domain::error::MyError;
use crate::domain::lupus::context::Tag;
use crate::domain::lupus::context_ext::{LupusCtxHelper, LupusHelpers};
use crate::domain::lupus::roles::{LupusAction, LupusRole};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[only_in(dms)]
pub async fn protect(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let target_tag: String = args.single()?;

    let (user_id, guild_id) = LupusCtxHelper::parse_id_to_guild_id(ctx, &msg.author.id).await?;

    let (target_id, _) = LupusCtxHelper::parse_tag_to_target_id(ctx, Tag(target_tag))
        .await
        .ok_or(MyError)?;

    let player = {
        let dt = ctx.data.read().await;
        dt.get_player(&guild_id, &user_id).await
    };

    if let Some(p) = player {
        if let LupusRole::BODYGUARD { .. } = *p.role() {
            LupusCtxHelper::send_lupus_command(ctx, msg, LupusAction::Protect(target_id)).await?
        } else {
            msg.channel_id
                .say(&ctx.http, "fra... controlla il ruolo va")
                .await?;
        }
    }
    Ok(())
}
