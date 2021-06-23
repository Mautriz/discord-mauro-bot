use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::domain::lupus::context_ext::LupusHelpers;
use crate::domain::msg_ext::MessageExt;

#[command]
#[only_in(guilds)]
pub async fn create(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let (user_id, guild_id) = msg.get_ids();

    // Unwrap is always safe, as LupusCtx is defined in the general context of the main application
    let rx_option = {
        let mut lupus_ctx = data.lupus_mut().await;
        lupus_ctx.create_game(&guild_id)
    };

    msg.channel_id
        .say(&ctx.http, format!("{}", "Partita creata con successo"))
        .await?;

    match rx_option {
        Ok(mut rx) => {
            let lupus_ctx = data.lupus().await;
            let rx_ref = &mut rx;
            lupus_ctx.handle_game(&guild_id, rx_ref, ctx).await;
        }
        _ => (),
    };

    Ok(())
}
