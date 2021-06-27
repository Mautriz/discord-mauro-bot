use serenity::prelude::TypeMap;
use tokio::sync::{RwLockReadGuard, RwLockWriteGuard};

use super::context::{LupusCtx, LupusManager, Tag};
use super::player::LupusPlayer;
use super::roles::LupusRole;
use serenity::async_trait;

use crate::domain::error::MyError;
use crate::domain::lupus::roles::LupusAction;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

pub struct LupusCtxHelper {}

impl LupusCtxHelper {
    pub async fn send_lupus_command(
        ctx: &Context,
        msg: &Message,
        action: LupusAction,
    ) -> CommandResult {
        let data = ctx.data.read().await;
        let lupus = data.lupus().await;
        let (user_id, guild_id) = LupusCtxHelper::parse_id_to_guild_id(ctx, &msg.author.id).await?;

        // Command handling
        if let Some(game) = lupus.get_game(&guild_id) {
            let mut game_writer = game.write().await;
            game_writer.push_night_action(user_id, action).await;
        } else {
            msg.channel_id
                .say(&ctx.http, format!("Game not found"))
                .await?;
        };

        Ok(())
    }

    pub async fn parse_tag_to_target_id(ctx: &Context, tag: Tag) -> Option<(UserId, GuildId)> {
        let data = ctx.data.read().await;
        let lupus = data.lupus().await;
        lupus.get_ids_from_tag(tag)
    }

    pub async fn parse_id_to_guild_id(
        ctx: &Context,
        uid: &UserId,
    ) -> Result<(UserId, GuildId), MyError> {
        let data = ctx.data.read().await;
        let lupus = data.lupus().await;
        let guild_id = lupus.user_id_to_guild_id(uid).ok_or(MyError)?;

        Ok((uid.to_owned(), guild_id.to_owned()))
    }

    pub async fn generic_action(
        ctx: &Context,
        msg: &Message,
        mut args: Args,
        role_check: fn(LupusRole) -> bool,
        action_create: fn(UserId) -> LupusAction,
    ) -> CommandResult {
        let target_tag: String = args.single()?;

        let (user_id, guild_id) = LupusCtxHelper::parse_id_to_guild_id(ctx, &msg.author.id).await?;
        let (target_id, _) = LupusCtxHelper::parse_tag_to_target_id(ctx, Tag(target_tag))
            .await
            .ok_or(MyError)?;

        let player = {
            let dt = ctx.data.read().await;
            dt.get_player(&guild_id, &user_id).await
        };

        if let Some(p) = player {
            if role_check(p.current_role().clone()) {
                LupusCtxHelper::send_lupus_command(ctx, msg, action_create(target_id)).await?;
                msg.channel_id
                    .say(&ctx.http, "azione registrata con successo")
                    .await?;
            } else {
                msg.channel_id
                    .say(&ctx.http, "fra... ruolo sbagliato")
                    .await?;
            }
        }
        Ok(())
    }
}

#[async_trait]
pub trait LupusHelpers {
    async fn lupus(&self) -> RwLockReadGuard<LupusManager>;
    async fn lupus_mut(&self) -> RwLockWriteGuard<LupusManager>;
    async fn get_player(&self, guild_id: &GuildId, uid: &UserId) -> Option<LupusPlayer>;
}

#[async_trait]
impl LupusHelpers for RwLockReadGuard<'_, TypeMap> {
    async fn lupus(&self) -> RwLockReadGuard<LupusManager> {
        self.get::<LupusCtx>().unwrap().read().await
    }

    async fn lupus_mut(&self) -> RwLockWriteGuard<LupusManager> {
        self.get::<LupusCtx>().unwrap().write().await
    }

    async fn get_player(&self, guild_id: &GuildId, uid: &UserId) -> Option<LupusPlayer> {
        let lupus = self.lupus().await;
        let game = lupus.get_game(guild_id)?;

        let gm_reader = game.read().await;
        let player = gm_reader.get_player(uid)?;
        Some(player.to_owned())
    }
}
