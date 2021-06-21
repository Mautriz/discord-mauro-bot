use std::{collections::HashMap, sync::Arc};

use super::roles::{LupusAction, LupusRole, Nature};
use super::roles_per_players;
use serenity::model::prelude::*;
use serenity::prelude::*;
use tokio::sync::mpsc::{channel, Receiver, Sender};

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

    pub fn create_game(&mut self, guild_id: &GuildId) -> Result<Receiver<GameMessage>, String> {
        if self.games.contains_key(guild_id) {
            Err(format!(
                "There's a game already in progress: {:?}",
                guild_id
            ))
        } else {
            let (tx, rx) = channel::<GameMessage>(10);
            let game = Arc::new(RwLock::new(LupusGame::new(tx)));
            self.games.insert(guild_id.to_owned(), game);
            Ok(rx)
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

    pub async fn handle_game(&self, guild_id: &GuildId, rx: &mut Receiver<GameMessage>) {
        match self.get_game(guild_id) {
            Some(game) => {
                let read_game = game.read().await;
                format!("Game handling started for game {:?}", guild_id);

                while let Some(msg) = rx.recv().await {
                    match msg {
                        GameMessage::DAYEND => {
                            read_game.handle_night(rx).await;
                        }
                        GameMessage::NIGHTEND => read_game.handle_day(rx).await,
                    };
                }
            }
            None => (),
        };
    }
}

#[derive(Clone)]
pub enum GameMessage {
    NIGHTEND,
    DAYEND,
}

#[derive(Debug)]
pub struct LupusGame {
    action_buffer: Vec<(UserId, LupusAction)>,
    joined_players: HashMap<UserId, LupusPlayer>,
    message_sender: Sender<GameMessage>,
}

impl LupusGame {
    fn new(tx: Sender<GameMessage>) -> Self {
        Self {
            message_sender: tx,
            action_buffer: vec![],
            joined_players: HashMap::new(),
        }
    }

    async fn handle_night(&self, rx: &mut Receiver<GameMessage>) {}

    async fn handle_day(&self, rx: &mut Receiver<GameMessage>) {}

    pub fn push_action(&mut self, user_id: UserId, cmd: LupusAction) {
        self.action_buffer.push((user_id, cmd))
    }

    fn process_action(&mut self, action: (UserId, LupusAction)) {
        match action.1 {
            _ => (),
        }
    }

    fn cleanup() {}
}

#[derive(Clone, Debug)]
struct LupusPlayer {
    role: LupusRole,
    alive: bool,
    framed: bool,
    role_blocked: bool,
    is_protected: bool,
    has_painting: bool,
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
        }
    }

    fn get_nature(&self) -> Nature {
        self.role.get_nature()
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

    fn cleanup(&mut self) {
        self.framed = false;
        self.has_painting = false;
        self.is_protected = false;
        self.role_blocked = false;
    }
}
