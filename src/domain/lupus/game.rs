use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::HashMap;
use std::convert::TryInto;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::Sender;

use super::context::GameMessage;
use super::player::KillError;
use super::player::LupusPlayer;
use super::roles::LupusAction;
use super::roles::LupusRole;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GamePhase {
    FIRSTNIGHT,
    NIGHT,
    DAY,
}

#[derive(Debug)]
pub struct LupusGame {
    action_buffer: HashMap<UserId, LupusAction>,
    pub joined_players: HashMap<UserId, LupusPlayer>,
    message_sender: Sender<GameMessage>,
    game_phase: GamePhase,
}

impl LupusGame {
    pub fn new(tx: Sender<GameMessage>) -> Self {
        Self {
            message_sender: tx,
            action_buffer: HashMap::new(),
            joined_players: HashMap::new(),
            game_phase: GamePhase::FIRSTNIGHT,
        }
    }

    pub fn set_phase(&mut self, phase: GamePhase) {
        self.game_phase = phase
    }

    pub fn get_phase(&self) -> &GamePhase {
        &self.game_phase
    }

    pub fn get_alive_players(&self) -> impl Iterator<Item = (&UserId, &LupusPlayer)> {
        self.joined_players.iter().filter(|(_, p)| p.alive())
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

        let mut required_uids = self
            .joined_players
            .iter()
            .filter(|(_uid, player)| player.role().can_action(&self.game_phase))
            .map(|(uid, _)| uid);

        if required_uids.all(|uid| self.action_buffer.iter().any(|(auid, _)| *auid == *uid)) {
            let _ = self.message_sender.send(GameMessage::NIGHTEND).await;
        }
    }

    pub async fn day_end(&self) {
        let has_ended = self.check_if_ended().await;
        if !has_ended {
            let _ = self.message_sender.send(GameMessage::DAYEND).await;
        }
    }

    pub fn is_phase(&self, phase: GamePhase) -> bool {
        self.game_phase == phase
    }

    pub async fn game_end(&self) -> Result<(), SendError<GameMessage>> {
        self.message_sender.send(GameMessage::GAMEEND).await
    }

    pub fn generic_kill_loop(
        &mut self,
        uid: UserId,
        killfn: fn(&mut LupusPlayer) -> Result<(), KillError>,
    ) -> Result<UserId, KillError> {
        let mut target_id = uid.to_owned();
        loop {
            let player_option = self.joined_players.get_mut(&target_id);
            if let Some(player) = player_option {
                if let Err(err) = killfn(player) {
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

    pub fn kill_loop(&mut self, uid: UserId) -> Result<UserId, KillError> {
        self.generic_kill_loop(uid, |user| user.kill())
    }

    pub fn guard_kill_loop(&mut self, uid: UserId) -> Result<UserId, KillError> {
        self.generic_kill_loop(uid, |user| user.guard_kill())
    }

    pub fn vote_kill_loop(&mut self, uid: UserId) -> Result<UserId, KillError> {
        self.generic_kill_loop(uid, |user| user.vote_kill())
    }

    pub async fn check_if_ended(&self) -> bool {
        let alive_players = self
            .joined_players
            .iter()
            .filter(|(_, player)| player.alive());

        let good_players_count = alive_players
            .clone()
            .filter(|(_, player)| player.role().is_actually_good())
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

    pub async fn process_actions(&mut self, ctx: &Context) {
        let mut buffer: Vec<_> = self.action_buffer.drain().collect();
        buffer.sort_by_key(|a| a.1);
        while let Some(action) = buffer.pop() {
            self.process_action(action, ctx).await
        }
    }

    pub async fn process_action(&mut self, action: (UserId, LupusAction), ctx: &Context) {
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
                if let Some(player) = self.joined_players.get_mut(&action.0) {
                    player.role = LupusRole::DOTTORE { has_healed: true }
                }
                if let Some(target) = self.joined_players.get_mut(&user_id) {
                    target.alive = true;
                }
            }
            LupusAction::GiveQuadro(target_id) => {
                if let Some(player) = self.joined_players.get_mut(&action.0) {
                    player.set_current_role(LupusRole::DORIANGREY {
                        has_quadro: true,
                        given_to: Some(target_id),
                    })
                }
            }
            LupusAction::GuardShot(user_id) => {
                let _ = self.guard_kill_loop(user_id);
            }
            LupusAction::Kill(user_id) | LupusAction::WolfVote(user_id) => {
                let _ = self.kill_loop(user_id);
            }
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

    pub fn cleanup(&mut self) {
        for (_, player) in self.joined_players.iter_mut() {
            player.cleanup()
        }
    }
}
