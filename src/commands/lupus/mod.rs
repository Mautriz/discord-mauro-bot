mod action;
mod create;
mod join;
mod leave;
mod ls;
mod start_game;
mod stats;
mod stop;

use action::*;
use create::*;
use join::*;
use leave::*;
use ls::*;
use start_game::*;
use stats::*;
use stop::*;

use serenity::framework::standard::macros::group;

#[group]
#[prefixes("lupus")]
#[commands(leave, stats, create, action, start_game, stop, join, ls)]
pub struct Lupus;
