use crate::domain::lupus::context_ext::LupusHelpers;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[only_in(guilds)]
#[description = "Fa partire il game con le persone joinate"]
pub async fn start(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let data = ctx.data.read().await;
    // Unwrap is always safe, as LupusCtx is defined in the general context of the main application
    let lupus_ctx = data.lupus().await;
    lupus_ctx.start_game(ctx, &guild_id).await;

    Ok(())
}
