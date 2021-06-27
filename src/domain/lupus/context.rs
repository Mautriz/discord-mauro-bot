use std::convert::TryInto;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::{collections::HashMap, sync::Arc};

use super::roles::{LupusAction, LupusRole, Nature};
use super::roles_per_players;
use serenity::model::prelude::*;
use serenity::prelude::*;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub struct LupusCtx {}

impl TypeMapKey for LupusCtx {
    type Value = Arc<RwLock<LupusManager>>;
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Tag(pub String);

pub struct LupusManager {
    games: HashMap<GuildId, Arc<RwLock<LupusGame>>>,
    user_to_guild: HashMap<UserId, GuildId>,
    tag_to_user_id: HashMap<Tag, UserId>,
}

impl LupusManager {
    pub fn new() -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            games: HashMap::new(),
            user_to_guild: HashMap::new(),
            tag_to_user_id: HashMap::new(),
        }))
    }

    pub async fn handle_night(&self, guild_id: &GuildId, ctx: &Context) {
        // Aspetta l'evento
        let game = self.get_game(guild_id).unwrap();
        let mut game_writer = game.write().await;
        game_writer.process_actions(ctx).await;
        game_writer.cleanup();
        let game_read = game.read().await;
        game_read.check_if_ended().await;
    }

    pub async fn handle_day(&self, guild_id: &GuildId) {
        let game = self.get_game(guild_id).unwrap();
        let game_read = game.read().await;
        game_read.check_if_ended().await;
    }

    pub fn get_ids_from_tag(&self, tag: Tag) -> Option<(UserId, GuildId)> {
        let user_id = *self.tag_to_user_id.get(&tag)?;
        let guild_id = *self.user_to_guild.get(&user_id)?;

        Some((user_id, guild_id))
    }

    pub fn user_id_to_guild_id(&self, uid: &UserId) -> Option<&GuildId> {
        self.user_to_guild.get(uid)
    }

    pub fn close_game(&mut self, guild_id: &GuildId) -> Option<Arc<RwLock<LupusGame>>> {
        let game = self.games.remove(guild_id)?;
        Some(game)
    }

    pub fn get_tag_from_id(&self, user_id: &UserId) -> Option<&Tag> {
        self.tag_to_user_id
            .iter()
            .find(|(_, t_id)| *user_id == **t_id)
            .map(|(tag, _)| tag)
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

    pub async fn add_user(&mut self, guild_id: &GuildId, player: &User) {
        // Da spostare - al momento il lock dura più di quanto dovrebbe (poco rilevante per pochi game)
        self.user_to_guild.insert(player.id, *guild_id);
        self.tag_to_user_id.insert(Tag(player.tag()), player.id);
        if let Some(game) = self.games.get(&guild_id) {
            let mut game_writer = game.write().await;
            game_writer
                .joined_players
                .insert(player.id, LupusPlayer::new());
            println!("{:?}", game_writer.joined_players.clone());
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

    pub async fn start_game(&self, ctx: &Context, guild_id: &GuildId) {
        if let Some(game) = self.games.get(&guild_id) {
            let mut game_writer = game.write().await;
            let player_num = game_writer.joined_players.len();
            let mut rng_roles = roles_per_players::get_roles(player_num);
            for (player_id, player) in game_writer.joined_players.iter_mut() {
                if let Some(role) = rng_roles.pop() {
                    println!("{:?} - {:?}", player.clone(), role.clone());
                    let ch = player_id.create_dm_channel(&ctx.http).await.unwrap();
                    player.role = role;

                    let _ = ch
                        .say(
                            &ctx.http,
                            format!("Il tuo ruolo è: {:?}", player.role.clone()),
                        )
                        .await;
                }
            }

            let wolfs_iter = game_writer
                .joined_players
                .iter()
                .filter(|(_a, b)| match b.role {
                    LupusRole::WOLF { .. } | LupusRole::GUFO => true,
                    _ => false,
                });

            let wolf_names = wolfs_iter.clone().map(|(uid, player)| {
                let tag = self.get_tag_from_id(uid).unwrap().0.clone();
                format!("tag: {:?}, role: {:?}", tag, player.role.clone())
            });

            let wolf_name_str = wolf_names.collect::<Vec<String>>().join(" - ");

            for (uid, _) in wolfs_iter.clone() {
                let ch = uid.create_dm_channel(&ctx.http).await.unwrap();
                let _ = ch
                    .say(
                        &ctx.http,
                        format!("I lupotti sono: {:?}", wolf_name_str.clone()),
                    )
                    .await;
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum GameMessage {
    // Chiamato quando tutti hanno fatto un'azione
    NIGHTEND,
    // Chiamato a fine voto
    DAYEND,
    GAMEEND,
}

#[derive(Debug)]
pub struct LupusGame {
    action_buffer: HashMap<UserId, LupusAction>,
    pub joined_players: HashMap<UserId, LupusPlayer>,
    message_sender: Sender<GameMessage>,
    is_first_night: bool,
}

impl LupusGame {
    fn new(tx: Sender<GameMessage>) -> Self {
        Self {
            message_sender: tx,
            action_buffer: HashMap::new(),
            joined_players: HashMap::new(),
            is_first_night: false,
        }
    }

    pub fn get_alive_players(&self) -> impl Iterator<Item = (&UserId, &LupusPlayer)> {
        self.joined_players.iter().filter(|(_, p)| p.alive)
    }

    pub fn get_alive_players_count(&self) -> u32 {
        self.get_alive_players().count().try_into().unwrap()
    }

    pub fn get_player(&self, player_id: &UserId) -> Option<&LupusPlayer> {
        self.joined_players.get(player_id)
    }

    pub fn get_player_mut(&mut self, player_id: &UserId) -> Option<&mut LupusPlayer> {
        self.joined_players.get_mut(player_id)
    }

    pub async fn push_night_action(&mut self, user_id: UserId, cmd: LupusAction) {
        self.action_buffer.insert(user_id, cmd);

        let required_uids: Vec<_> = if self.is_first_night {
            self.joined_players
                .iter()
                .filter(|(_uid, player)| player.role().can_action_fist_night())
                .map(|(uid, _)| uid)
                .collect()
        } else {
            self.joined_players
                .iter()
                .filter(|(_uid, player)| player.role().can_action())
                .map(|(uid, _)| uid)
                .collect()
        };

        if required_uids
            .iter()
            .all(|uid| self.action_buffer.iter().any(|(auid, _)| *auid == **uid))
        {
            self.is_first_night = false;
            let _ = self.message_sender.send(GameMessage::NIGHTEND).await;
        }
    }

    pub async fn day_end(&self) {
        let has_ended = self.check_if_ended().await;
        if !has_ended {
            let _ = self.message_sender.send(GameMessage::DAYEND).await;
        }
    }

    pub async fn game_end(&self) -> Result<(), SendError<GameMessage>> {
        self.message_sender.send(GameMessage::GAMEEND).await
    }

    pub fn kill_loop(&mut self, uid: UserId) {
        let mut target_id = uid.to_owned();
        loop {
            let player_option = self.joined_players.get_mut(&target_id);
            if let Some(player) = player_option {
                if let Err(KillError::DorianGray { target }) = player.kill() {
                    target_id = target;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    pub fn guard_kill_loop(&mut self, uid: UserId) {
        let mut target_id = uid.to_owned();
        loop {
            let player_option = self.joined_players.get_mut(&target_id);
            if let Some(player) = player_option {
                if let Err(KillError::DorianGray { target }) = player.guard_kill() {
                    target_id = target;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    pub fn vote_kill_loop(&mut self, uid: UserId) -> Result<UserId, KillError> {
        let mut target_id = uid.to_owned();
        loop {
            let player_option = self.joined_players.get_mut(&target_id);
            if let Some(player) = player_option {
                if let Err(err) = player.vote_kill() {
                    if let KillError::DorianGray { target } = err {
                        target_id = target;
                    } else {
                        return Err(err);
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Ok(target_id)
    }

    async fn check_if_ended(&self) -> bool {
        let alive_players = self
            .joined_players
            .iter()
            .filter(|(_, player)| player.alive);

        let good_players_count = alive_players
            .clone()
            .filter(|(_, player)| player.role.is_actually_good())
            .count();

        // Vincono i lupi o vince il rimanente
        if alive_players.count() <= 1 || good_players_count == 0 {
            let result = self.game_end().await;
            if let Err(_err) = result {
                println!("Non sono riuscito a terminare il game con successo");
            }
            true
        } else {
            false
        }
    }

    async fn process_actions(&mut self, ctx: &Context) {
        let mut buffer: Vec<_> = self.action_buffer.drain().collect();
        buffer.sort_by_key(|a| a.1);
        while let Some(action) = buffer.pop() {
            self.process_action(action, ctx).await
        }
    }

    async fn process_action(&mut self, action: (UserId, LupusAction), ctx: &Context) {
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
            LupusAction::Heal(user_id) => {
                if let Some(target) = self.joined_players.get_mut(&user_id) {
                    target.alive = true;
                }
            }
            LupusAction::GuardShot(user_id) => self.guard_kill_loop(user_id),
            LupusAction::Kill(user_id) | LupusAction::WolfVote(user_id) => self.kill_loop(user_id),
            LupusAction::Protect(user_id) => {
                if let Some(target) = self.joined_players.get_mut(&user_id) {
                    if let LupusRole::BODYGUARD { self_protected } = target.role {
                        if self_protected {
                            return;
                        }
                        target.role = LupusRole::BODYGUARD {
                            self_protected: true,
                        };
                    }
                    target.is_protected = true;
                }
            }
            LupusAction::RoleBlock(user_id) => {
                if let Some(target) = self.joined_players.get_mut(&user_id) {
                    target.role_blocked = true;
                }
            }
            LupusAction::TrueSight(user_id) => {
                let channel = action.0.create_dm_channel(&ctx.http).await.unwrap();
                let player_to_check = self.joined_players.get(&user_id);

                if let Some(pl) = player_to_check {
                    let nature = pl.get_nature();
                    let _ = channel
                        .say(
                            &ctx.http,
                            format!("Fra conta che quello che stai guardando è {}", nature),
                        )
                        .await;
                }
            }
            LupusAction::Pass => (),
        }
    }

    fn cleanup(&mut self) {
        for (_, player) in self.joined_players.iter_mut() {
            player.cleanup()
        }
    }
}

#[derive(Clone, Debug)]
pub struct LupusPlayer {
    role: LupusRole,
    alive: bool,
    framed: bool,
    role_blocked: bool,
    is_protected: bool,
}

#[derive(Debug)]
pub enum KillError {
    DorianGray { target: UserId },
    UnkillableTarget,
    IsProtected,
}

impl Display for KillError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self)
    }
}
impl Error for KillError {}

impl LupusPlayer {
    fn new() -> Self {
        Self {
            alive: true,
            role: Default::default(),
            is_protected: false,
            framed: false,
            role_blocked: false,
        }
    }

    pub fn get_nature(&self) -> Nature {
        if self.framed {
            Nature::EVIL
        } else {
            self.role.get_nature()
        }
    }

    pub fn role(&self) -> &LupusRole {
        &self.role
    }

    pub fn current_role(&self) -> &LupusRole {
        match &self.role {
            LupusRole::STREGA(inner) => inner,
            _ => &self.role(),
        }
    }

    pub fn set_current_role(&mut self, new_role: LupusRole) {
        match self.role {
            LupusRole::STREGA(..) => {
                self.role = LupusRole::STREGA(Box::new(new_role));
            }
            _ => self.role = new_role,
        }
    }

    pub fn kill(&mut self) -> Result<(), KillError> {
        if self.is_protected {
            return Err(KillError::IsProtected);
        }
        match self.current_role().to_owned() {
            LupusRole::DORIANGREY {
                given_to: Some(quadro_target),
                has_quadro: true,
            } => {
                self.set_current_role(LupusRole::DORIANGREY {
                    given_to: None,
                    has_quadro: false,
                });

                Err(KillError::DorianGray {
                    target: quadro_target.clone(),
                })
            }
            LupusRole::CRICETO => Err(KillError::UnkillableTarget),
            LupusRole::SERIALKILLER => Err(KillError::UnkillableTarget),
            _ => {
                self.alive = false;
                Ok(())
            }
        }
    }

    pub fn guard_kill(&mut self) -> Result<(), KillError> {
        if self.is_protected {
            return Err(KillError::IsProtected);
        }
        match self.current_role().to_owned() {
            LupusRole::DORIANGREY {
                given_to: Some(quadro_target),
                has_quadro: true,
            } => {
                self.set_current_role(LupusRole::DORIANGREY {
                    given_to: None,
                    has_quadro: false,
                });
                Err(KillError::DorianGray {
                    target: quadro_target.clone(),
                })
            }
            LupusRole::SERIALKILLER => Err(KillError::UnkillableTarget),
            _ => {
                self.alive = false;
                Ok(())
            }
        }
    }

    pub fn vote_kill(&mut self) -> Result<(), KillError> {
        match self.current_role().to_owned() {
            LupusRole::DORIANGREY {
                given_to: Some(quadro_target),
                has_quadro: true,
            } => {
                self.set_current_role(LupusRole::DORIANGREY {
                    given_to: None,
                    has_quadro: false,
                });
                Err(KillError::DorianGray {
                    target: quadro_target.clone(),
                })
            }
            _ => {
                self.alive = false;
                Ok(())
            }
        }
    }

    fn cleanup(&mut self) {
        self.framed = false;
        self.is_protected = false;
        self.role_blocked = false;
        match self.role.clone() {
            LupusRole::STREGA(_) => {
                self.role = LupusRole::STREGA(Box::new(LupusRole::NOTASSIGNED));
            }
            _ => (),
        }
    }
}
