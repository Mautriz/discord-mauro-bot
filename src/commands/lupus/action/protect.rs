use crate::domain::lupus::context_ext::LupusCtxHelper;
use crate::domain::lupus::roles::{LupusAction, LupusRole};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[only_in(dms)]
#[description = "Bodyguard: protegge un player"]
pub async fn protect(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    LupusCtxHelper::generic_action(
        ctx,
        msg,
        args,
        |role| matches!(role, LupusRole::BODYGUARD { .. }),
        |target_id| LupusAction::Protect(target_id),
    )
    .await
}
