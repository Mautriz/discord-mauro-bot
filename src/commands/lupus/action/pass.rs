use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use crate::domain::lupus::{context_ext::LupusCtxHelper, roles::LupusAction};

#[command]
#[description = "Salta il turno se non sai che fare"]
pub async fn pass(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    LupusCtxHelper::send_lupus_command(ctx, msg, LupusAction::Pass).await?;

    Ok(())
}
