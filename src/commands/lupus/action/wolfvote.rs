use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::domain::lupus::context_ext::LupusCtxHelper;
use crate::domain::lupus::roles::LupusAction;

#[command]
pub async fn wolfvote(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let target_id: UserId = args.single()?;

    LupusCtxHelper::send_lupus_command(ctx, msg, LupusAction::WolfVote(target_id)).await
}
