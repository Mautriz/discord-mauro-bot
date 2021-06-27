use crate::domain::lupus::context_ext::LupusCtxHelper;
use crate::domain::lupus::roles::{LupusAction, LupusRole};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[only_in(dms)]
pub async fn shoot(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    LupusCtxHelper::generic_action(
        ctx,
        msg,
        args,
        |role| matches!(role, LupusRole::VIGILANTE { has_shot: false }),
        |target_id| LupusAction::GuardShot(target_id),
    )
    .await
}
