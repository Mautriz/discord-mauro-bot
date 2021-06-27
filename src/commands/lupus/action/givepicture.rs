use crate::domain::error::MyError;
use crate::domain::lupus::context::Tag;
use crate::domain::lupus::context_ext::{LupusCtxHelper, LupusHelpers};
use crate::domain::lupus::roles::LupusRole;
use crate::domain::msg_ext::MessageExt;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[only_in(dms)]
pub async fn givepicture(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let target_tag: String = args.single()?;
    let (user_id, guild_id) = msg.get_ids();
    let (target_id, _) = LupusCtxHelper::parse_tag_to_target_id(ctx, Tag(target_tag))
        .await
        .ok_or(MyError)?;

    {
        let dt = ctx.data.read().await;
        let lupus = dt.lupus().await;
        let game = lupus.get_game(&guild_id).ok_or(MyError)?;

        let mut game_writer = game.write().await;
        let player = game_writer.get_player_mut(&user_id).ok_or(MyError)?;

        if let LupusRole::DORIANGREY {
            has_quadro: true, ..
        } = *player.current_role()
        {
            player.set_current_role(LupusRole::DORIANGREY {
                has_quadro: true,
                given_to: Some(target_id),
            })
        } else {
            msg.channel_id
                .say(&ctx.http, "fra... ruolo sbagliato")
                .await?;
        }
    };

    Ok(())
}
