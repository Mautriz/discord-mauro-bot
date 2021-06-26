use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

mod frame;
mod givepicture;
mod heal;
mod kill;
mod possess;
mod protect;
mod roleblock;
mod shoot;
mod start_vote;
mod truesight;
mod wolfvote;

use frame::*;
use givepicture::*;
use heal::*;
use kill::*;
use possess::*;
use protect::*;
use roleblock::*;
use shoot::*;
use start_vote::*;
use truesight::*;
use wolfvote::*;

#[command]
#[sub_commands(
    roleblock,
    frame,
    givepicture,
    protect,
    kill,
    wolfvote,
    truesight,
    possess,
    start_vote,
    heal,
    shoot
)]
pub async fn action(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let _ = msg
        .channel_id
        .say(&ctx.http, format!("Please specify an action"))
        .await;
    Ok(())
}
