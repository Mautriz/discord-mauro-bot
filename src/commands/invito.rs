use std::time::Duration;

use serenity::framework::standard::{macros::command, Args, CommandResult};

use serenity::model::prelude::*;
use serenity::prelude::*;

use tokio::time::sleep;

#[command]
#[description = "crea un invito temporaneo assurdo che dopo 10 minuti esplode"]
pub async fn invito(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let channel = ctx.cache.guild_channel(msg.channel_id).await;

    if let None = channel {
        return Ok(());
    }

    let creation = channel
        .unwrap()
        .create_invite(&ctx, |i| i.max_age(600).temporary(true))
        .await;

    let invite = match creation {
        Ok(invite) => invite,
        Err(why) => {
            println!("Err creating invite: {:?}", why);
            if let Err(why) = msg.channel_id.say(&ctx, "Error creating invite").await {
                println!("Err sending err msg: {:?}", why);
            }

            return Ok(());
        }
    };

    let content = format!("BEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEM {}", invite.url());
    let sent_message = msg.channel_id.say(&ctx, &content).await;

    if let Ok(msg) = sent_message {
        sleep(Duration::from_secs(600)).await;
        let _ = msg.delete(&ctx.http).await;
    }

    Ok(())
}
