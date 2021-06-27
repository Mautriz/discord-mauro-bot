use serenity::model::prelude::*;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

use super::roles::LupusRole;
use super::roles::Nature;

#[derive(Clone, Debug)]
pub struct LupusPlayer {
    pub role: LupusRole,
    pub alive: bool,
    pub framed: bool,
    pub role_blocked: bool,
    pub is_protected: bool,
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
    pub fn new() -> Self {
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

    pub fn alive(&self) -> bool {
        self.alive
    }

    pub fn cleanup(&mut self) {
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
