use std::collections::HashMap;
use std::convert::TryInto;
use std::sync::Arc;
use std::time::Duration;

use serenity::collector::ReactionAction;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::futures::StreamExt;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use tracing::info;

use crate::consts::*;
use crate::domain::error::MyError;
use crate::domain::lupus::context::Tag;
use crate::domain::lupus::context_ext::{LupusCtxHelper, LupusHelpers};
use crate::domain::lupus::game::GamePhase;
use crate::domain::lupus::roles::LupusRole;
use crate::domain::msg_ext::MessageExt;

#[command]
#[only_in(guilds)]
pub async fn builder_test(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // let target_tag: String = args.single()?;
    // let (target_id, target_guild_id) =
    //     LupusCtxHelper::parse_tag_to_target_id(ctx, Tag(target_tag.clone()))
    //         .await
    //         .ok_or(MyError)?;

    // let data_read = ctx.data.read().await;
    // let lupus_manager = data_read.lupus().await;
    // let (_user_id, guild_id) = msg.get_ids();
    // let game = lupus_manager.get_game(&guild_id).ok_or(MyError)?;

    // if guild_id != target_guild_id {
    //     println!("Different guild ids");
    //     return Ok(());
    // }

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

    // let game_phase = *game.read().await.get_phase();
    // if !matches!(game_phase, GamePhase::DAY) {
    //     msg.channel_id
    //         .say(
    //             &ctx.http,
    //             format!(
    //                 "Il voto puo' essere fatto solo di giorno, fase attuale: {:?}",
    //                 game_phase
    //             ),
    //         )
    //         .await?;

    //     return Ok(());

    // MessageBuilder::new().
    let sent_msg = msg.channel_id.say(&ctx.http, format!("AO")).await?;

    let number_of_player_alive: usize = 16; //  game.read().await.get_alive_players_count();

    // game.write().await.set_phase(GamePhase::VOTAZIONE);

    for i in 0.. {
        sent_msg
            .react(&ctx.http, num_emojis.get(i).ok_or(MyError)?.to_owned())
            .await?;
    }

    let reacts = sent_msg
        .await_reactions(ctx)
        .collect_limit(number_of_player_alive.try_into().unwrap())
        .timeout(Duration::from_secs(60))
        .await
        .collect::<Vec<Arc<ReactionAction>>>()
        .await;

    let result_map: HashMap<ReactionType, i32> = reacts
        .into_iter()
        .filter_map(|a| match *a {
            ReactionAction::Added(ref react) => Some((**react).clone()),
            ReactionAction::Removed(_) => None,
        })
        .fold(HashMap::new(), |mut map, reaction| {
            map.insert(
                reaction.emoji.clone(),
                *map.get(&reaction.emoji).unwrap_or(&0) + 1,
            );
            map
        });

    let (_, &highest) = result_map.iter().max_by_key(|(_, &b)| b).ok_or(MyError)?;
    let number_of_highest = result_map.iter().filter(|(_, &num)| num == highest).count();

    if number_of_highest > 1 {
        let sent_msg = msg
            .channel_id
            .say(&ctx.http, format!("C'è stato un pareggio, nessuno muore"))
            .await?;
    } else {
    }

    // let yes_count = *result_map
    //     .get(&ReactionType::Unicode(YES_CIRCLE.to_string()))
    //     .unwrap_or(&0);
    // let no_count = *result_map
    //     .get(&ReactionType::Unicode(NO_CIRCLE.to_string()))
    //     .unwrap_or(&0);

    // let result = if yes_count > no_count {
    //     YES_CIRCLE
    // } else if yes_count < no_count {
    //     NO_CIRCLE
    // } else {
    //     ASTENUTO_CIRCLE
    // };

    // if result == YES_CIRCLE {
    //     let killed_player = {
    //         let mut game_writer = game.write().await;
    //         let killed_id = game_writer.vote_kill_loop(target_id)?;
    //         game_writer
    //             .get_player(&killed_id)
    //             .ok_or(MyError)?
    //             .to_owned()
    //     };

    //     let game_reader = game.read().await;
    //     let maybe_player = game_reader
    //         .get_alive_players()
    //         .find(|(_, player)| matches!(player.role(), &LupusRole::MEDIUM))
    //         .map(|(uid, _)| uid);

    //     if let Some(player) = maybe_player {
    //         let ch = player.create_dm_channel(&ctx.http).await?;
    //         ch.say(
    //             &ctx.http,
    //             format!(
    //                 "fra vedi che il tizio morto era {:?}",
    //                 killed_player.get_nature()
    //             ),
    //         )
    //         .await?;
    //     }
    // }

    // game.read().await.day_end().await;

    // msg.channel_id
    //     .say(
    //         &ctx.http,
    //         format!("risultato per: {} ... {}", target_tag, result),
    //     )
    //     .await?;

    Ok(())
}
