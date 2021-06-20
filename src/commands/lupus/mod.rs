mod action;
mod create;
mod join;
mod start;
mod stats;
mod stop;

use action::*;
use create::*;
use join::*;
use serenity::framework::standard::macros::group;
use start::*;
use stats::*;
use stop::*;

#[group]
#[prefixes("lupus")]
#[commands(stats, start, create, action, start, stop, join)]
pub struct Lupus;
