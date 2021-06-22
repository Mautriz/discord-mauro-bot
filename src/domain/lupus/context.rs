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

    pub async fn add_user(&mut self, guild_id: &GuildId, player_id: &UserId) {
        // Da spostare - al momento il lock dura più di quanto dovrebbe (poco rilevante per pochi game)
        self.user_to_guild
            .insert(player_id.to_owned(), guild_id.to_owned());
        if let Some(game) = self.games.get(&guild_id) {
            let mut game_writer = game.write().await;
            game_writer
                .joined_players
                .insert(player_id.clone(), LupusPlayer::new());
        }
    }

    pub async fn remove_user(&mut self, guild_id: &GuildId, player_id: &UserId) {
        self.user_to_guild.remove(player_id);
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
                format!("Game handling started for game {:?}", guild_id);
                while let Some(msg) = rx.recv().await {
                    match msg {
                        GameMessage::DAYEND => {
                            self.handle_night(game, rx).await;
                        }
                        GameMessage::NIGHTEND => self.handle_day(game, rx).await,
                        GameMessage::GAMEEND => return,
                    };
                }
            }
            None => (),
        };
    }

    async fn handle_night(&self, game: &Arc<RwLock<LupusGame>>, rx: &mut Receiver<GameMessage>) {
        let mut game_writer = game.write().await;
        game_writer.process_actions();
        game_writer.cleanup();
    }

    async fn handle_day(&self, game: &Arc<RwLock<LupusGame>>, rx: &mut Receiver<GameMessage>) {}
}

#[derive(Clone)]
pub enum GameMessage {
    NIGHTEND,
    DAYEND,
    GAMEEND,
}

#[derive(Debug)]
pub struct LupusGame {
    action_buffer: HashMap<UserId, LupusAction>,
    joined_players: HashMap<UserId, LupusPlayer>,
    message_sender: Sender<GameMessage>,
}

impl LupusGame {
    fn new(tx: Sender<GameMessage>) -> Self {
        Self {
            message_sender: tx,
            action_buffer: HashMap::new(),
            joined_players: HashMap::new(),
        }
    }

    pub fn push_action(&mut self, user_id: UserId, cmd: LupusAction) {
        self.action_buffer.insert(user_id, cmd);
    }

    fn process_actions(&mut self) {
        let mut buffer: Vec<_> = self.action_buffer.drain().collect();
        buffer.sort_by_key(|a| a.1);
        while let Some(action) = buffer.pop() {
            self.process_action(action)
        }
    }

    fn process_action(&mut self, action: (UserId, LupusAction)) {
        let player = self.joined_players.get(&action.0);
        // Se il player che fa l'azione è roleblockato, ritorna senza fare nulla
        if let Some(LupusPlayer {
            role_blocked: true, ..
        }) = player
        {
            return;
        }

        match action.1 {
            LupusAction::Frame(user_id) => {
                if let Some(target) = self.joined_players.get_mut(&user_id) {
                    target.framed = true;
                }
            }
            LupusAction::GivePicture(user_id) => {
                if let Some(target) = self.joined_players.get_mut(&user_id) {
                    target.has_painting = true;
                }
            }
            LupusAction::Heal(user_id) => {
                if let Some(target) = self.joined_players.get_mut(&user_id) {
                    target.alive = true;
                }
            }
            LupusAction::Kill(user_id) => {
                if let Some(player) = self.joined_players.get_mut(&user_id) {
                    player.alive = false;
                }
            }
            LupusAction::Protect(user_id) => {
                if let Some(target) = self.joined_players.get_mut(&user_id) {
                    target.is_protected = true;
                }
            }
            LupusAction::SelfProtect => {
                if let Some(LupusPlayer {
                    role: LupusRole::BODYGUARD { self_protected },
                    is_protected,
                    ..
                }) = self.joined_players.get_mut(&action.0)
                {
                    *is_protected = true;
                    *self_protected = true;
                }
            }
            LupusAction::RoleBlock(user_id) => {
                if let Some(target) = self.joined_players.get_mut(&user_id) {
                    target.role_blocked = true;
                }
            }
            _ => (),
        }
    }

    fn cleanup(&mut self) {
        for (_, player) in self.joined_players.iter_mut() {
            player.cleanup()
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
