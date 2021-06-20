use rand::Rng;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use rand::prelude::*;

#[command]
pub async fn random(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let bottom_limit = args.single::<u32>()?;
    let top_limit = args.single::<u32>()?;

    if top_limit < bottom_limit {
        let _ = msg
            .channel_id
            .say(&ctx.http, format!("Oh fra vedi di mette bene sti numeri"))
            .await;
        return Ok(());
    }

    let random_number = {
        let mut rng = thread_rng();
        rng.gen_range(bottom_limit..=top_limit)
    };

    let _ = msg
        .channel_id
        .say(
            &ctx.http,
            format!("Il numero random uscito è {}", random_number),
        )
        .await;

    Ok(())
}
