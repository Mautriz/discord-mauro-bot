use crate::domain::error::MyError;
use crate::domain::lupus::context::Tag;
use crate::domain::lupus::context_ext::{LupusCtxHelper, LupusHelpers};
use crate::domain::lupus::roles::{LupusAction, LupusRole};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[only_in(dms)]
pub async fn protect(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    LupusCtxHelper::generic_action(
        ctx,
        msg,
        args,
        |role| matches!(role, LupusRole::BODYGUARD { .. }),
        |target_id| LupusAction::Protect(target_id),
    )
    .await
}
