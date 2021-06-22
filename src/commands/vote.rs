use crate::consts::{ASTENUTO_CIRCLE, NO_CIRCLE, YES_CIRCLE};
use serenity::collector::ReactionAction;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::futures::StreamExt;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

#[command]
#[only_in(guilds)]
pub async fn vote(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let requested_votes: u32 = args.single()?;
    let remains = args.rest();

    let sent_msg = msg
        .channel_id
        .say(&ctx.http, format!("votazione per: {}", remains))
        .await?;

    sent_msg.react(&ctx.http, YES_CIRCLE).await?;
    sent_msg.react(&ctx.http, NO_CIRCLE).await?;
    sent_msg.react(&ctx.http, ASTENUTO_CIRCLE).await?;

    let reacts = sent_msg
        .await_reactions(ctx)
        .collect_limit(requested_votes)
        .timeout(Duration::from_secs(30))
        .await
        .collect::<Vec<Arc<ReactionAction>>>()
        .await;

    let result_map = reacts
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

    msg.channel_id
        .say(
            &ctx.http,
            format!("risultato per: {} ... {}", remains, result),
        )
        .await?;

    Ok(())
}
