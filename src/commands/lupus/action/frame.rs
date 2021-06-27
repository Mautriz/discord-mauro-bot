use crate::domain::error::MyError;
use crate::domain::lupus::context::{LupusPlayer, Tag};
use crate::domain::lupus::context_ext::{LupusCtxHelper, LupusHelpers};
use crate::domain::lupus::roles::{LupusAction, LupusRole};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use tracing::info;

#[command]
#[only_in(dms)]
pub async fn frame(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    LupusCtxHelper::generic_action(
        ctx,
        msg,
        args,
        |rl| matches!(rl, LupusRole::GUFO),
        |uid| LupusAction::Frame(uid),
    )
    .await
}
