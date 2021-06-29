use crate::domain::error::MyError;
use crate::domain::lupus::context::Tag;
use crate::domain::lupus::context_ext::{LupusCtxHelper, LupusHelpers};
use crate::domain::lupus::game::GamePhase;
use crate::domain::lupus::player::LupusPlayer;
use crate::domain::lupus::roles::{LupusAction, LupusRole};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "Gufo: mostra un player come cattivone al veggente e killa un omino"]
#[only_in(dms)]
pub async fn frame_and_kill(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let frame_target: String = args.single()?;
    let kill_target: String = args.single()?;

    let (user_id, guild_id) = LupusCtxHelper::parse_id_to_guild_id(ctx, &msg.author.id).await?;
    let (frame_target_id, _) = LupusCtxHelper::parse_tag_to_target_id(ctx, Tag(frame_target))
        .await
        .ok_or(MyError)?;

    let (kill_target_id, _) = LupusCtxHelper::parse_tag_to_target_id(ctx, Tag(kill_target))
        .await
        .ok_or(MyError)?;

    let (game_phase, p): (GamePhase, LupusPlayer) = {
        let dt = ctx.data.read().await;
        let lupus = dt.lupus().await;
        let game = lupus.get_game(&guild_id).ok_or(MyError)?.read().await;
        let player = game.get_player(&user_id).cloned().ok_or(MyError)?;
        let game_phase: &GamePhase = game.get_phase();
        let gp = (*game_phase).clone();
        (gp, player)
    };

    let player_role = p.current_role().clone();
    if matches!(player_role, LupusRole::GUFO { is_leader: true })
        && player_role.can_action(&game_phase)
    {
        LupusCtxHelper::send_lupus_command(
            ctx,
            msg,
            LupusAction::FrameAndKill(frame_target_id, kill_target_id),
        )
        .await?;
        msg.channel_id
            .say(&ctx.http, "azione registrata con successo")
            .await?;
    } else {
        msg.channel_id
            .say(
                &ctx.http,
                "ruolo sbagliato o momento sbagliato per fare action",
            )
            .await?;
    }
    Ok(())
}
