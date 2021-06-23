mod action;
mod create;
mod join;
mod leave;
mod ls;
mod start;
mod stats;
mod stop;

use action::*;
use create::*;
use join::*;
use leave::*;
use ls::*;
use serenity::framework::standard::macros::group;
use start::*;
use stats::*;
use stop::*;

#[group]
#[prefixes("lupus")]
#[commands(leave, stats, start, create, action, start, stop, join, ls)]
pub struct Lupus;
