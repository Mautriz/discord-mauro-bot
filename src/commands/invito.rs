use std::time::Duration;

use serenity::framework::standard::{macros::command, Args, CommandResult};

use serenity::model::prelude::*;
use serenity::prelude::*;

use tokio::time::sleep;

const INVITE_DURATION: u64 = 600;

#[command]
#[only_in(guilds)]
#[sub_commands(perma)]
#[description = "crea un invito temporaneo assurdo che dopo 10 minuti esplode"]
pub async fn invito(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let channel = ctx.cache.guild_channel(msg.channel_id).await;

    if let None = channel {
        return Ok(());
    }

    let creation = channel
        .unwrap()
        .create_invite(&ctx, |i| i.max_age(INVITE_DURATION).temporary(true))
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

    // Elimina il messaggio con il link dopo 10 minuti (quindi quando è scaduto)
    if let Ok(bot_msg) = sent_message {
        sleep(Duration::from_secs(INVITE_DURATION)).await;
        let _ = bot_msg.delete(&ctx.http).await;
        let _ = msg.delete(&ctx.http).await;
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
#[required_permissions(ADMINISTRATOR)]
#[description = "crea un invito temporaneo assurdo che dopo 10 minuti esplode"]
pub async fn perma(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let channel = ctx.cache.guild_channel(msg.channel_id).await;
    if let None = channel {
        return Ok(());
    }

    let creation = channel
        .unwrap()
        .create_invite(&ctx, |i| i.max_uses(1).max_age(INVITE_DURATION))
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

    let content = format!("Invito permanente {}", invite.url());
    let sent_message = msg.channel_id.say(&ctx, &content).await;

    // Elimina il messaggio con il link dopo 10 minuti (quindi quando è scaduto)
    if let Ok(bot_msg) = sent_message {
        sleep(Duration::from_secs(INVITE_DURATION)).await;
        let _ = bot_msg.delete(&ctx.http).await;
        let _ = msg.delete(&ctx.http).await;
    }

    Ok(())
}
