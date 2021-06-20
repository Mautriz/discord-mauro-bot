use std::collections::HashSet;
use std::{collections::HashMap, sync::Arc};

use serenity::model::prelude::*;
use serenity::prelude::*;

use super::roles::{LupusAction, LupusRole, Nature};
use super::roles_per_players;

pub struct LupusCtx {}

impl TypeMapKey for LupusCtx {
    type Value = Arc<RwLock<LupusManager>>;
}

#[derive(Clone)]
pub struct LupusManager {
    games: HashMap<GuildId, Arc<RwLock<LupusGame>>>,
    user_to_guild: HashMap<UserId, GuildId>,
}

impl LupusManager {
    pub fn new() -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            games: HashMap::new(),
            user_to_guild: HashMap::new(),
        }))
    }

    pub fn create_game(&mut self, guild_id: GuildId) -> String {
        if self.games.contains_key(&guild_id) {
            format!("There's a game already in progress: {:?}", guild_id)
        } else {
            let game = Arc::new(RwLock::new(LupusGame::new()));
            self.games.insert(guild_id, game);
            format!("Game created successfully")
        }
    }

    pub async fn add_user(&self, guild_id: &GuildId, player_id: &UserId) {
        if let Some(game) = self.games.get(&guild_id) {
            let mut game_writer = game.write().await;
            game_writer
                .joined_players
                .insert(player_id.clone(), LupusPlayer::new());
        }
    }

    pub async fn remove_user(&self, guild_id: &GuildId, player_id: &UserId) {
        if let Some(game) = self.games.get(&guild_id) {
            let mut game_writer = game.write().await;
            game_writer.joined_players.remove(player_id);
        }
    }

    pub fn get_game(&self, guild_id: &GuildId) -> Option<&Arc<RwLock<LupusGame>>> {
        self.games.get(&guild_id)
    }

    pub async fn start_game(&self, guild_id: &GuildId) {
        if let Some(game) = self.games.get(&guild_id) {
            let mut game_writer = game.write().await;
            let player_num = game_writer.joined_players.len();
            let mut rng_roles = roles_per_players::get_roles(player_num);
            for (_, player) in game_writer.joined_players.iter_mut() {
                if let Some(role) = rng_roles.pop() {
                    player.role = role;
                }
            }
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct LupusGame {
    action_buffer: Vec<(UserId, LupusAction)>,
    joined_players: HashMap<UserId, LupusPlayer>,
}

impl LupusGame {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn push_action(&mut self, user_id: UserId, cmd: LupusAction) {
        self.action_buffer.push((user_id, cmd))
    }

    fn process_action(&mut self, action: (UserId, LupusAction)) {
        match action.1 {
            _ => (),
        }
    }
}

#[derive(Clone, Debug)]
struct LupusPlayer {
    role: LupusRole,
    alive: bool,
    framed: bool,
    role_blocked: bool,
    is_protected: bool,
    has_painting: bool,
    special_role: LupusRole,
}

enum KillError {
    HasPainting,
}

impl LupusPlayer {
    fn new() -> Self {
        Self {
            alive: true,
            role: Default::default(),
            is_protected: false,
            framed: false,
            role_blocked: false,
            has_painting: false,
            special_role: LupusRole::NOT_ASSIGNED,
        }
    }

    fn get_nature(&self) -> Nature {
        if self.special_role != LupusRole::NOT_ASSIGNED {
            self.special_role.get_nature()
        } else {
            self.role.get_nature()
        }
    }

    fn kill(&mut self) -> Result<(), KillError> {
        match self {
            Self {
                has_painting: true, ..
            } => Err(KillError::HasPainting),
            _ => {
                self.alive = false;
                Ok(())
            }
        }
    }
}
