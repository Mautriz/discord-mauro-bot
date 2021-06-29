use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use serenity::collector::ReactionAction;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::futures::StreamExt;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::consts::{ASTENUTO_CIRCLE, NO_CIRCLE, YES_CIRCLE};
use crate::domain::error::MyError;
use crate::domain::lupus::context::Tag;
use crate::domain::lupus::context_ext::{LupusCtxHelper, LupusHelpers};
use crate::domain::lupus::game::GamePhase;
use crate::domain::lupus::roles::LupusRole;
use crate::domain::msg_ext::MessageExt;

#[command]
#[only_in(guilds)]
pub async fn start_vote(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let target_tag: String = args.single()?;
    let (target_id, target_guild_id) =
        LupusCtxHelper::parse_tag_to_target_id(ctx, Tag(target_tag.clone()))
            .await
            .ok_or(MyError)?;

    let data_read = ctx.data.read().await;
    let lupus_manager = data_read.lupus().await;
    let (_user_id, guild_id) = msg.get_ids();
    let game = lupus_manager.get_game(&guild_id).ok_or(MyError)?;

    if guild_id != target_guild_id {
        println!("Different guild ids");
        return Ok(());
    }

    let game_phase = *game.read().await.get_phase();
    if !matches!(game_phase, GamePhase::DAY) {
        msg.channel_id
            .say(
                &ctx.http,
                format!(
                    "Il voto puo' essere fatto solo di giorno, fase attuale: {:?}",
                    game_phase
                ),
            )
            .await?;
    }

    let number_of_player_alive = game.read().await.get_alive_players_count();

    let sent_msg = msg
        .channel_id
        .say(&ctx.http, format!("votazione per: {}", target_tag.clone()))
        .await?;

    game.write().await.set_phase(GamePhase::VOTAZIONE);

    sent_msg.react(&ctx.http, YES_CIRCLE).await?;
    sent_msg.react(&ctx.http, NO_CIRCLE).await?;
    sent_msg.react(&ctx.http, ASTENUTO_CIRCLE).await?;

    let reacts = sent_msg
        .await_reactions(ctx)
        .collect_limit(number_of_player_alive.into())
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

    let yes_count = *result_map
        .get(&ReactionType::Unicode(YES_CIRCLE.to_string()))
        .unwrap_or(&0);
    let no_count = *result_map
        .get(&ReactionType::Unicode(NO_CIRCLE.to_string()))
        .unwrap_or(&0);

    let result = if yes_count > no_count {
        YES_CIRCLE
    } else if yes_count < no_count {
        NO_CIRCLE
    } else {
        ASTENUTO_CIRCLE
    };

    if result == YES_CIRCLE {
        let killed_player = {
            let mut game_writer = game.write().await;
            let killed_id = game_writer.vote_kill_loop(target_id)?;
            game_writer
                .get_player(&killed_id)
                .ok_or(MyError)?
                .to_owned()
        };

        let game_reader = game.read().await;
        let maybe_player = game_reader
            .get_alive_players()
            .find(|(_, player)| matches!(player.role(), &LupusRole::MEDIUM))
            .map(|(uid, _)| uid);

        if let Some(player) = maybe_player {
            let ch = player.create_dm_channel(&ctx.http).await?;
            ch.say(
                &ctx.http,
                format!(
                    "fra vedi che il tizio morto era {:?}",
                    killed_player.get_nature()
                ),
            )
            .await?;
        }
    }

    game.read().await.day_end().await;

    msg.channel_id
        .say(
            &ctx.http,
            format!("risultato per: {} ... {}", target_tag, result),
        )
        .await?;

    Ok(())
}
