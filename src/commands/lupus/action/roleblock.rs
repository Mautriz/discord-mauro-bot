use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::domain::lupus::context_ext::LupusHelpers;
use crate::domain::lupus::roles::LupusAction;
use crate::domain::msg_ext::MessageExt;

#[command]
pub async fn roleblock(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let target_id: UserId = args.single()?;
    let data = ctx.data.read().await;
    let lupus = data.lupus().await;
    let (user_id, guild_id) = msg.get_ids();

    // Command handling
    if let Some(game) = lupus.get_game(&guild_id) {
        let mut game_writer = game.write().await;
        game_writer
            .push_night_action(user_id, LupusAction::RoleBlock(target_id))
            .await;
    } else {
        msg.channel_id
            .say(&ctx.http, format!("Game not found"))
            .await?;
    }

    Ok(())
}
