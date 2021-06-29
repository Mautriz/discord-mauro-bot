use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::domain::error::MyError;
use crate::domain::lupus::context::GameMessage;
use crate::domain::lupus::context_ext::LupusHelpers;
use crate::domain::msg_ext::MessageExt;

#[command]
#[only_in(guilds)]
pub async fn create(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let (_, guild_id) = msg.get_ids();

    // let user = user_id.to_user(&ctx.http).await?;
    // let dm_channel = user.create_dm_channel(&ctx.http).await?;

    // Unwrap is always safe, as LupusCtx is defined in the general context of the main application
    let mut rx = {
        let mut lupus_ctx = data.lupus_mut().await;
        lupus_ctx.create_game(&guild_id).unwrap()
    };

    msg.channel_id
        .say(&ctx.http, "Partita creata con successo")
        .await?;

    while let Some(game_message) = rx.recv().await {
        println!("msg: {:?}", game_message.clone());
        match game_message {
            GameMessage::NIGHTEND => {
                let lupus = data.lupus().await;
                lupus.handle_night(&guild_id, ctx).await;
                msg.channel_id.say(&ctx.http, "La notte è finita").await?;
            }
            GameMessage::DAYEND => {
                let lupus = data.lupus().await;
                lupus.handle_day(&guild_id).await;
                msg.channel_id.say(&ctx.http, "Il giorno è finito").await?;
            }
            GameMessage::GAMEEND => {
                let game = {
                    let mut lupus = data.lupus_mut().await;
                    let game = lupus.close_game(&guild_id).ok_or(MyError)?;

                    game.to_owned()
                };

                let gm_reader = game.read().await;
                let lupus = data.lupus().await;

                let mapped_players: Vec<_> = gm_reader
                    .joined_players
                    .iter()
                    .map(|(a, b)| (lupus.get_tag_from_id(a).unwrap().0.clone(), b))
                    .collect();

                msg.channel_id
                    .say(
                        &ctx.http,
                        format!(
                            "La partita è finita, questi sono tutti i player: {:?}",
                            mapped_players
                        ),
                    )
                    .await?;
            }
        };
    }

    Ok(())
}
