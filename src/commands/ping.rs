use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::domain::error::MyError;

#[command]
pub async fn ping(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let url = msg.author.avatar_url().ok_or(MyError)?;
    let _sent_msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.thumbnail(url)
                    .color((200,20,20))
                    .title("Votazione per il giorno")
                    .description(
                        "Una descrizione matta :black_medium_small_square: *ciao* `AO bella fra`\nCIAOOOOOOOOOO :black_medium_small_square: \nCIAOOOOOOOOOO :black_medium_small_square: \nCIAOOOOOOOOOO :black_medium_small_square: \nCIAOOOOOOOOOO :black_medium_small_square: \nCIAOOOOOOOOOO :black_medium_small_square:`\nCIAOOOOOOOOOO :black_medium_small_square:`\nCIAOOOOOOOOOO :black_medium_small_square:",
                    )
                    .footer(|a| a.text("Bella neri TIEMME"))
            })
        })
        .await?;

    Ok(())
}
