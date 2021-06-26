use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::domain::lupus::context::GameMessage;
use crate::domain::lupus::context_ext::LupusHelpers;
use crate::domain::msg_ext::MessageExt;

#[command]
#[only_in(guilds)]
pub async fn create(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let (user_id, guild_id) = msg.get_ids();

    let user = user_id.to_user(&ctx.http).await?;
    // let dm_channel = user.create_dm_channel(&ctx.http).await?;

    // Unwrap is always safe, as LupusCtx is defined in the general context of the main application
    let mut rx = {
        let mut lupus_ctx = data.lupus_mut().await;
        lupus_ctx.create_game(&guild_id).unwrap()
    };

    msg.channel_id
        .say(&ctx.http, format!("Partita creata con successo"))
        .await?;

    while let Some(msg) = rx.recv().await {
        println!("msg: {:?}", msg.clone());
        match msg {
            GameMessage::DAYEND => {
                let lupus = data.lupus().await;
                lupus.handle_night(&guild_id, ctx).await;
            }
            GameMessage::NIGHTEND => {
                let lupus = data.lupus().await;
                lupus.handle_day(&guild_id).await;
            }
            GameMessage::GAMEEND => (),
        };
    }

    Ok(())
}
