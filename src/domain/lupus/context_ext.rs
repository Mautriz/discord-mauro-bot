use serenity::prelude::TypeMap;
use tokio::sync::{RwLockReadGuard, RwLockWriteGuard};

use super::context::{LupusCtx, LupusManager, LupusPlayer, Tag};
use serenity::async_trait;

use crate::domain::lupus::roles::LupusAction;
use crate::domain::msg_ext::MessageExt;
use serenity::framework::standard::CommandResult;
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
        let (user_id, guild_id) = msg.get_ids();

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
