use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::domain::error::MyError;
use crate::domain::lupus::context::GameMessage;
use crate::domain::lupus::context_ext::LupusHelpers;
use crate::domain::msg_ext::MessageExt;

use crate::consts::*;

#[command]
#[only_in(guilds)]
#[description = "Crea la partita lupus"]
pub async fn create(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let num_emojis: Vec<ReactionType> = vec![
        ReactionType::Unicode(ONE.into()),
        ReactionType::Unicode(TWO.into()),
        ReactionType::Unicode(THREE.into()),
        ReactionType::Unicode(FOUR.into()),
        ReactionType::Unicode(FIVE.into()),
        ReactionType::Unicode(SIX.into()),
        ReactionType::Unicode(SEVEN.into()),
        ReactionType::Unicode(EIGHT.into()),
        ReactionType::Unicode(NINE.into()),
        ReactionType::Unicode(TEN.into()),
        ReactionType::Unicode(ELEVEN.into()),
        ReactionType::Unicode(TWELVE.into()),
        ReactionType::Unicode(THIRTEEN.into()),
        ReactionType::Unicode(FOURTEEN.into()),
        ReactionType::Unicode(FIFTEEN.into()),
        ReactionType::Unicode(SIXTEEN.into()),
    ];

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
            GameMessage::HANDLENIGHT => {
                let lupus = data.lupus().await;
                lupus.handle_night(&guild_id, ctx, msg).await;
                msg.channel_id.say(&ctx.http, "La notte è finita").await?;
            }
            GameMessage::HANDLEVOTATION => {
                let lupus = data.lupus().await;
                lupus.handle_votation(ctx, msg, &guild_id).await?;
            }
            GameMessage::HANDLEDAY => {
                let lupus = data.lupus().await;
                lupus.handle_day(&guild_id, ctx).await;
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
