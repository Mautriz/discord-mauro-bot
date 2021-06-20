use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn join(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let _ = msg.channel_id.say(&ctx.http, format!("")).await;
    Ok(())
}
