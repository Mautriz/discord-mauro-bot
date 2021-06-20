use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

fn exctract_game(ctx: &Context, msg: &Message) {}

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
    let _ = msg
        .channel_id
        .say(&ctx.http, format!("Please specify an action"))
        .await;
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

// pub enum LupusNightCommand {
//     RoleBlock { user_id: UserId },
//     Frame { user_id: UserId },
//     GivePicture { user_id: UserId },
//     Protect { user_id: UserId },

//     Kill { user_id: UserId },
//     WolfVote { user_id: UserId },
//     TrueSight { user_id: UserId },
//     Heal { user_id: UserId },
//     Remember { user_id: UserId },
//     // Possess { user_id: UserId },
// }
