use crate::domain::error::MyError;
use crate::domain::lupus::context::Tag;
use crate::domain::lupus::context_ext::LupusCtxHelper;
use crate::domain::lupus::roles::LupusAction;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn givepicture(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let target_tag: String = args.single()?;
    let (target_id, _target_guild) = LupusCtxHelper::parse_tag_to_target_id(ctx, Tag(target_tag))
        .await
        .ok_or(MyError)?;

    LupusCtxHelper::send_lupus_command(ctx, msg, LupusAction::GivePicture(target_id)).await
}
