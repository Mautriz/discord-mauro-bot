use crate::domain::lupus::context_ext::LupusCtxHelper;
use crate::domain::lupus::roles::{LupusAction, LupusRole};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[only_in(dms)]
#[description = "Dorian Grey: da un quadro a qualcuno, se dorian muore il quadro esplode insieme all'altro tizio, dorian rimane in vita"]
pub async fn givepicture(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    LupusCtxHelper::generic_action(
        ctx,
        msg,
        args,
        |a| {
            matches!(
                a,
                LupusRole::DORIANGREY {
                    has_quadro: false,
                    ..
                }
            )
        },
        |a| LupusAction::GiveQuadro(a),
    )
    .await
}
