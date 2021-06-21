use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::domain::lupus::context_ext::LupusHelpers;
use crate::domain::lupus::roles::LupusAction;
use crate::domain::msg_ext::MessageExt;

#[command]
#[sub_commands(
    roleblock,
    frame,
    givepicture,
    protect,
    kill,
    wolfvote,
    truesight,
    remember,
    possess
)]
pub async fn action(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let _ = msg
        .channel_id
        .say(&ctx.http, format!("Please specify an action"))
        .await;
    Ok(())
}

#[command]
pub async fn roleblock(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let lupus = data.lupus().await;
    let (user_id, guild_id) = msg.get_ids();

    // Command handling
    if let Some(game) = lupus.get_game(&guild_id) {
        let mut game_writer = game.write().await;
        game_writer.push_action(user_id, LupusAction::RoleBlock(user_id))
    } else {
        msg.channel_id
            .say(&ctx.http, format!("Game not found"))
            .await?;
    }

    Ok(())
}

#[command]
pub async fn frame(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let _ = msg
        .channel_id
        .say(&ctx.http, format!("Please specify an action"))
        .await;
    Ok(())
}

#[command]
pub async fn givepicture(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let _ = msg
        .channel_id
        .say(&ctx.http, format!("Please specify an action"))
        .await;
    Ok(())
}

#[command]
pub async fn protect(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let _ = msg
        .channel_id
        .say(&ctx.http, format!("Please specify an action"))
        .await;
    Ok(())
}

#[command]
pub async fn kill(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let _ = msg
        .channel_id
        .say(&ctx.http, format!("Please specify an action"))
        .await;
    Ok(())
}

#[command]
pub async fn wolfvote(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let _ = msg
        .channel_id
        .say(&ctx.http, format!("Please specify an action"))
        .await;
    Ok(())
}

#[command]
pub async fn truesight(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let _ = msg
        .channel_id
        .say(&ctx.http, format!("Please specify an action"))
        .await;
    Ok(())
}

#[command]
pub async fn remember(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let _ = msg
        .channel_id
        .say(&ctx.http, format!("Please specify an action"))
        .await;
    Ok(())
}

#[command]
pub async fn possess(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let _ = msg
        .channel_id
        .say(&ctx.http, format!("Please specify an action"))
        .await;
    Ok(())
}
