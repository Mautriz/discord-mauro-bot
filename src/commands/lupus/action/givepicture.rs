use crate::domain::error::MyError;
use crate::domain::lupus::context::Tag;
use crate::domain::lupus::context_ext::{LupusCtxHelper, LupusHelpers};
use crate::domain::lupus::roles::{LupusAction, LupusRole};
use crate::domain::msg_ext::MessageExt;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn givepicture(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let target_tag: String = args.single()?;
    let (user_id, guild_id) = msg.get_ids();
    let (target_id, _) = LupusCtxHelper::parse_tag_to_target_id(ctx, Tag(target_tag))
        .await
        .ok_or(MyError)?;

    let player = {
        let dt = ctx.data.read().await;
        dt.get_player(&guild_id, &user_id).await
    };

    if let Some(p) = player {
        if let LupusRole::DORIANGREY {
            has_quadro: true, ..
        } = *p.current_role()
        {
            LupusCtxHelper::send_lupus_command(ctx, msg, LupusAction::GivePicture(target_id))
                .await?
        } else {
            msg.channel_id
                .say(&ctx.http, "fra... ruolo sbagliato")
                .await?;
        }
    }
    Ok(())
}
