use crate::domain::lupus::context_ext::LupusCtxHelper;
use crate::domain::lupus::roles::{LupusAction, LupusRole};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "Gufo: mostra un player come cattivone al veggente"]
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
