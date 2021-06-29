use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::domain::lupus::context_ext::LupusHelpers;
use crate::domain::msg_ext::MessageExt;

#[command]
#[only_in(guilds)]
#[description = "Esci dalla partita, se ancora non e' iniziata"]
pub async fn leave(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let (user_id, guild_id) = msg.get_ids();

    {
        let data = ctx.data.read().await;
        // Unwrap is always safe, as LupusCtx is defined in the general context of the main application
        let mut lupus_ctx = data.lupus_mut().await;
        lupus_ctx.remove_user(&guild_id, &user_id).await
    };

    msg.channel_id
        .say(&ctx.http, format!("Join completed successfully"))
        .await?;
    Ok(())
}
