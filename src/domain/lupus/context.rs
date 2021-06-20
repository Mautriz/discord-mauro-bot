use std::collections::HashSet;
use std::{collections::HashMap, sync::Arc};

use serenity::model::prelude::*;
use serenity::prelude::*;

use super::roles::{LupusCommand, LupusRole};

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

    async fn add_user(&self, guild_id: &GuildId, player_id: UserId) {
        if let Some(game) = self.games.get(&guild_id) {
            let mut game_writer = game.write().await;
            game_writer.joined_players.insert(player_id);
        }
    }

    async fn remove_user(&self, guild_id: &GuildId, player_id: &UserId) {
        if let Some(game) = self.games.get(&guild_id) {
            let mut game_writer = game.write().await;
            game_writer.joined_players.remove(player_id);
        }
    }

    fn get_game(&self, guild_id: &GuildId) -> Option<&Arc<RwLock<LupusGame>>> {
        self.games.get(&guild_id)
    }
}

#[derive(Clone, Default)]
struct LupusGame {
    command_buffer: Vec<LupusCommand>,
    joined_players: HashSet<UserId>,
    time: LupusTime,
}

impl LupusGame {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn process_commands(&mut self) {
        // self.command_buffer
    }
}
#[derive(Clone)]
struct LupusDay {}
#[derive(Clone)]
struct LupusNight {}

#[derive(Clone)]
enum LupusTime {
    Day(LupusDay),
    Night(LupusNight),
}

impl Default for LupusTime {
    fn default() -> Self {
        LupusTime::Day(LupusDay {})
    }
}

struct LupusPlayer {
    role: LupusRole,
    user_id: UserId,
    is_alive: bool,
}
