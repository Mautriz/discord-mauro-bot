use std::{collections::HashMap, sync::Arc};

use serenity::model::prelude::*;
use serenity::prelude::*;

#[derive(Clone)]
pub struct LupusCtx {
    games: HashMap<GuildId, Arc<RwLock<LupusGame>>>,
}

impl LupusCtx {
    fn create_game(&mut self, guild_id: GuildId) -> String {
        if self.games.contains_key(&guild_id) {
            format!("There's a game already in progress: {:?}", guild_id)
        } else {
            let game = Arc::new(RwLock::new(LupusGame::new()));
            self.games.insert(guild_id, game);
            format!("Game created successfully")
        }
    }

    fn add_user(&mut self, player_id: UserId) {}

    fn get_game(&self, guild_id: GuildId) {}
}

#[derive(Clone)]
pub enum Role {}

#[derive(Clone)]
pub enum Actions {}

#[derive(Clone, Default)]
struct LupusGame {
    action_buffer: Vec<Actions>,
    player_list: Vec<UserId>,
}

impl LupusGame {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
