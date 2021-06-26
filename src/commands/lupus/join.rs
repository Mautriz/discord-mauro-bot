use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::domain::lupus::context_ext::LupusHelpers;

#[command]
#[only_in(guilds)]
pub async fn join(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    // let user_id = msg.author.id;
    let guild_id = msg.guild_id.unwrap();

    {
        let data = ctx.data.read().await;
        // Unwrap is always safe, as LupusCtx is defined in the general context of the main application
        let mut lupus_ctx = data.lupus_mut().await;
        lupus_ctx.add_user(&guild_id, &msg.author).await;
    };

    let _ = msg
        .channel_id
        .say(&ctx.http, format!("Join completed successfully"))
        .await;
    Ok(())
}
