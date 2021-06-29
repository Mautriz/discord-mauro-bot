use std::time::Duration;
use std::{collections::HashMap, sync::Arc};

use crate::domain::lupus::player::LupusPlayer;

use serenity::collector::message_collector::MessageCollectorBuilder;
use serenity::futures::StreamExt;
use tokio::sync::RwLockWriteGuard;

use super::game::{GamePhase, LupusGame};
use super::roles::LupusRole;
use super::roles_per_players;
use serenity::model::prelude::*;
use serenity::prelude::*;
use tokio::sync::mpsc::{channel, Receiver};

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

    pub async fn handle_wolf_reassing(
        ctx: &Context,
        game_writer: &mut RwLockWriteGuard<'_, LupusGame>,
    ) {
        if let Some(new_wolf_leader) = game_writer.reassign_wolf_if_master_is_dead() {
            if let Ok(ch) = new_wolf_leader.create_dm_channel(&ctx.http).await {
                let _ = ch
                    .say(
                        &ctx.http,
                        "Sei il nuovo capo dei lupi, ora sarai tu a dover killare",
                    )
                    .await;
            }
        };
    }

    pub async fn handle_night(&self, guild_id: &GuildId, ctx: &Context, msg: &Message) {
        // Aspetta l'evento
        let game = self.get_game(guild_id).unwrap();
        {
            let mut game_writer = game.write().await;

            let mut killed_player_ids: Vec<_> = game_writer.process_actions(ctx).await.collect();

            // Da controllare se funziona
            let player_to_kill = self
                .handle_vigilante_death(ctx, killed_player_ids.clone(), &game_writer)
                .await;
            if let Some(p_to_kill) = player_to_kill {
                if let Ok(uid) = game_writer.guard_kill_loop(p_to_kill) {
                    killed_player_ids.push(uid)
                }
            }

            let killed_players: Vec<_> = killed_player_ids
                .iter()
                .filter_map(|a| self.get_tag_from_id(&a))
                .map(|a| &a.0)
                .collect();

            LupusManager::handle_wolf_reassing(ctx, &mut game_writer).await;

            let _ = msg
                .channel_id
                .say(
                    &ctx.http,
                    format!(
                        "I player sterminati distrutti uccisi demoliti massacrati sono: {:?}",
                        killed_players
                    ),
                )
                .await;

            game_writer.cleanup();
            game_writer.set_phase(GamePhase::DAY);
        }
        let game_read = game.read().await;
        game_read.check_if_ended().await;
    }

    pub async fn handle_day(&self, guild_id: &GuildId, ctx: &Context) {
        let game = self.get_game(guild_id).unwrap();
        {
            let mut game_writer = game.write().await;
            game_writer.set_phase(GamePhase::NIGHT)
        }
        let game_read = game.read().await;
        LupusManager::handle_wolf_reassing(ctx, &mut game.write().await).await;
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
                    LupusRole::WOLF { .. } | LupusRole::GUFO { .. } => true,
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

    pub async fn handle_vigilante_death(
        &self,
        ctx: &Context,
        killed_player_ids: Vec<UserId>,
        game_writer: &RwLockWriteGuard<'_, LupusGame>,
    ) -> Option<UserId> {
        if let Some((uid, _)) = killed_player_ids
            .iter()
            .filter_map(|id| {
                if let Some(player) = game_writer.get_player(&id) {
                    Some((id, player))
                } else {
                    None
                }
            })
            .find(|(_, p)| {
                matches!(
                    p,
                    LupusPlayer {
                        role: LupusRole::VIGILANTE { has_shot: false },
                        ..
                    }
                )
            })
        {
            if let Ok(ch) = uid.create_dm_channel(&ctx.http).await {
                if let Ok(_) = ch
                    .say(
                        &ctx.http,
                        "Devi sparare il tuo colpo, scrivi chi vuoi uccidere",
                    )
                    .await
                {
                    let mut response = MessageCollectorBuilder::new(&ctx)
                        .author_id(uid.0)
                        .channel_id(ch.id)
                        .collect_limit(1)
                        .timeout(Duration::from_secs(30))
                        .await;

                    if let Some(msg) = response.next().await {
                        if msg.content.clone().to_lowercase() == "none".to_string() {
                            return None;
                        }
                        let (uid, _) = self.get_ids_from_tag(Tag(msg.content.clone()))?;

                        return Some(uid);
                    }
                }
            }
        }

        return None;
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
