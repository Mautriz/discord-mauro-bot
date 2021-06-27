use crate::domain::lupus::context_ext::LupusCtxHelper;
use crate::domain::lupus::roles::{LupusAction, LupusRole};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[only_in(dms)]
pub async fn kill(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    LupusCtxHelper::generic_action(
        ctx,
        msg,
        args,
        |role| matches!(role, LupusRole::SERIALKILLER),
        |target_id| LupusAction::Kill(target_id),
    )
    .await
}
