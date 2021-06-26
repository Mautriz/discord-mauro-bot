use std::time::Duration;

use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn ping(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let sent_msg = msg
        .channel_id
        .say(
            &ctx.http,
            format!("Ciao {}, sei un pagliaccio", msg.author.tag()),
        )
        .await?;

    let dm = msg.author.id.create_dm_channel(&ctx.http).await?;

    let _ = dm.say(&ctx.http, "ciaooo").await;

    while let Some(reaction) = sent_msg
        .await_reaction(&ctx)
        .timeout(Duration::from_secs(30))
        .await
    {
        msg.react(
            &ctx.http,
            ReactionType::Unicode("2\u{fe0f}\u{20e3}".to_string()),
        )
        .await?;
        println!("{:?}", reaction);
    }

    Ok(())
}
