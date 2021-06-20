mod action;
mod create;
mod join;
mod start;
mod stop;

use action::*;
use create::*;
use serenity::framework::standard::macros::group;
use start::*;
use stop::*;

#[group]
#[prefixes("lupus")]
#[commands(start, create, action, start, stop)]
pub struct Lupus;
