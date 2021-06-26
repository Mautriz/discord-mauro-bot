use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::domain::error::MyError;
use crate::domain::lupus::context::Tag;
use crate::domain::lupus::context_ext::{LupusCtxHelper, LupusHelpers};
use crate::domain::msg_ext::MessageExt;

#[command]
pub async fn possess(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let target_tag: String = args.single()?;
    let (target_id, _) = LupusCtxHelper::parse_tag_to_target_id(ctx, Tag(target_tag))
        .await
        .ok_or(MyError)?;

    let (user_id, guild_id) = msg.get_ids();
    let data = ctx.data.read().await;
    let lupus = data.lupus().await;
    let game = lupus.get_game(&guild_id).ok_or(MyError)?;

    let game_reader = game.read().await;
    let target_player = game_reader.get_player(&target_id).ok_or(MyError)?;

    let role = {
        let mut game_writer = game.write().await;
        // let target_player = game_writer.get_player(&target_id).ok_or(MyError)?;
        let player_mut = game_writer.get_player_mut(&user_id).ok_or(MyError)?;
        player_mut.set_witch_role(target_player.current_role().to_owned());

        target_player.current_role().to_owned()
    };

    let ch = msg.author.create_dm_channel(&ctx.http).await?;

    ch.say(&ctx.http, format!("Il ruolo che hai preso è: {:?}", role))
        .await?;
    Ok(())
}
