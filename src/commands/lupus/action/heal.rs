use crate::domain::lupus::context_ext::LupusCtxHelper;
use crate::domain::lupus::roles::{LupusAction, LupusRole};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[only_in(dms)]
#[description = "Medino: fa rinascere un player, una sola volta per game"]
pub async fn heal(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    LupusCtxHelper::generic_action(
        ctx,
        msg,
        args,
        |rl| matches!(rl, LupusRole::DOTTORE { has_healed: false }),
        |uid| LupusAction::Heal(uid),
    )
    .await
}
