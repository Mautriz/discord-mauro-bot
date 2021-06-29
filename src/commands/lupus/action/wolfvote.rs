use crate::domain::lupus::context_ext::LupusCtxHelper;
use crate::domain::lupus::roles::{LupusAction, LupusRole};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[only_in(dms)]
#[description = "Wolf: killa un playerone (puo' essere usato solo dal wolf master)"]
pub async fn wolfvote(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    LupusCtxHelper::generic_action(
        ctx,
        msg,
        args,
        |role| matches!(role, LupusRole::WOLF { is_leader: true }),
        |target_id| LupusAction::WolfVote(target_id),
    )
    .await
}
