use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::domain::lupus::context::Tag;
use crate::domain::lupus::context_ext::LupusHelpers;
use crate::domain::msg_ext::MessageExt;

#[derive(Debug)]
struct MyError;

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyError")
    }
}

impl Error for MyError {}

#[command]
pub async fn possess(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let tag: String = args.single()?;
    let (user_id, guild_id) = msg.get_ids();
    let data = ctx.data.read().await;
    let lupus = data.lupus().await;
    let (target_id, _) = lupus.get_ids_from_tag(Tag(tag)).ok_or(MyError)?;
    let game = lupus.get_game(&guild_id).ok_or(MyError)?;

    {
        let mut game_writer = game.write().await;
        let target_player = game_writer.get_player(&target_id).ok_or(MyError)?;
        let player_mut = game_writer.get_player_mut(&user_id).ok_or(MyError)?;
        player_mut.set_witch_role(target_player.role())
    }

    let _ = msg
        .channel_id
        .say(&ctx.http, format!("Please specify an action"))
        .await;
    Ok(())
}
