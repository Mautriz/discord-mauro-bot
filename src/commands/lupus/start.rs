use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::domain::lupus::context::LupusCtx;

#[command]
pub async fn start(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let data = ctx.data.read().await;
    // Unwrap is always safe, as LupusCtx is defined in the general context of the main application
    let lupus_ctx = data.get::<LupusCtx>().unwrap().read().await;
    lupus_ctx.start_game(&guild_id).await;
    Ok(())
}
