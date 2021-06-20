use std::any::Any;

use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::domain::lupus::context::LupusCtx;

#[command]
pub async fn create(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let message = {
        let data = ctx.data.read().await;
        // Unwrap is always safe, as LupusCtx is defined in the general context of the main application
        let mut lupus_ctx = data.get::<LupusCtx>().unwrap().write().await;
        lupus_ctx.create_game(msg.guild_id.unwrap())
    };

    msg.channel_id
        .say(&ctx.http, format!("{:?}", message))
        .await?;

    Ok(())
}
