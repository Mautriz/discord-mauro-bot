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

use crate::consts::*;
use crate::domain::error::MyError;

#[command]
#[only_in(guilds)]
pub async fn builder_test(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
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

    let sent_msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.thumbnail("url")
                    .title("Votazione per il giorno")
                    .description("Una descrizione matta")
                    .footer(|a| a.text("Bella neri TIEMME"))
            })
        })
        .await?;

    let number_of_player_alive: usize = 16;

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
        let _ = msg
            .channel_id
            .say(&ctx.http, format!("C'è stato un pareggio, nessuno muore"))
            .await?;
    } else {
    }

    Ok(())
}
