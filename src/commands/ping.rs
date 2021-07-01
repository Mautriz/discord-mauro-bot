use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn ping(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    msg.channel_id
        .send_message(&ctx.http, |a| a.tts(true).content("Provaaa"))
        .await?;

    Ok(())
    // let _sent_msg = msg
    //     .channel_id
    //     .send_message(&ctx.http, |m| {
    //         m.embed(|e| {
    //             e.thumbnail(url)
    //                 .color((200,20,20))
    //                 .title("Votazione per il giorno")
    //                 .description(
    //                     "Una descrizione matta :black_medium_small_square: *ciao* `AO bella fra`\nCIAOOOOOOOOOO :black_medium_small_square: \nCIAOOOOOOOOOO :black_medium_small_square: \nCIAOOOOOOOOOO :black_medium_small_square: \nCIAOOOOOOOOOO :black_medium_small_square: \nCIAOOOOOOOOOO :black_medium_small_square:`\nCIAOOOOOOOOOO :black_medium_small_square:`\nCIAOOOOOOOOOO :black_medium_small_square:",
    //                 )
    //                 .footer(|a| a.text("Bella neri TIEMME"))
    //         })
    //     })
    //     .await?;

    // Ok(())
}
