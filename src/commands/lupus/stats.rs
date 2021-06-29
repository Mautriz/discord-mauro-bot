use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::domain::lupus::context::LupusCtx;

#[command]
#[owners_only]
#[description = "Solo per mauro, mostra tutte le stat del game"]
pub async fn stats(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();

    let lupus_game = {
        let data = ctx.data.read().await;
        // Unwrap is always safe, as LupusCtx is defined in the general context of the main application
        let lupus_lock = data.get::<LupusCtx>().unwrap().read().await;
        match lupus_lock.get_game(&guild_id) {
            Some(game) => {
                let game_owned = game.read().await;
                Some(format!("{:?}", game_owned))
            }
            _ => None,
        }
    };

    if let Some(game) = lupus_game {
        msg.channel_id.say(&ctx.http, game).await?;
    } else {
        msg.channel_id
            .say(&ctx.http, format!("Game non trovato"))
            .await?;
    }

    Ok(())
}
