use crate::domain::lupus::context_ext::{LupusCtxHelper, LupusHelpers};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn ls(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let (_user_id, guild_id) = LupusCtxHelper::parse_id_to_guild_id(ctx, &msg.author.id).await?;
    let data = ctx.data.read().await;
    // Unwrap is always safe, as LupusCtx is defined in the general context of the main application
    let lupus_ctx = data.lupus().await;
    if let Some(game) = lupus_ctx.get_game(&guild_id) {
        let game_reader = game.read().await;
        let player_tag_list: Vec<_> = game_reader
            .joined_players
            .iter()
            .map(|a| lupus_ctx.get_tag_from_id(a.0).map(|a| &a.0))
            .filter(|a| matches!(a, Some(_)))
            .map(|a| a.unwrap())
            .collect();

        msg.channel_id
            .say(&ctx.http, format!("Players in game: {:?}", player_tag_list))
            .await?;
    }

    Ok(())
}
