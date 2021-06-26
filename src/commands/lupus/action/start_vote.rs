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
    let game = lupus_manager
        .get_game(&guild_id)
        .ok_or("Ciao".to_string())?;

    if guild_id != target_guild_id {
        println!("Different guild ids");
        return Ok(());
    }

    let number_of_player_alive = game.read().await.get_alive_players_count();

    let sent_msg = msg
        .channel_id
        .say(&ctx.http, format!("votazione per: {}", target_tag.clone()))
        .await?;

    sent_msg.react(&ctx.http, YES_CIRCLE).await?;
    sent_msg.react(&ctx.http, NO_CIRCLE).await?;
    sent_msg.react(&ctx.http, ASTENUTO_CIRCLE).await?;

    let reacts = sent_msg
        .await_reactions(ctx)
        .collect_limit(number_of_player_alive.into())
        .timeout(Duration::from_secs(30))
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
        let mut game_writer = game.write().await;
        let player = game_writer.get_player_mut(&target_id).ok_or(MyError)?;
        let _ = player.force_kill();

        YES_CIRCLE
    } else if yes_count < no_count {
        NO_CIRCLE
    } else {
        ASTENUTO_CIRCLE
    };

    game.read().await.day_end().await;

    msg.channel_id
        .say(
            &ctx.http,
            format!("risultato per: {} ... {}", target_tag, result),
        )
        .await?;

    Ok(())
}
