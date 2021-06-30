mod action;
mod create;
mod join;
mod leave;
mod ls;
mod start_game;
mod stats;
mod stop;

use action::{
    frame::*, frame_and_kill::*, givepicture::*, heal::*, kill::*, pass::*, possess::*, protect::*,
    roleblock::*, shoot::*, truesight::*, wolfvote::*,
};
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
#[commands(
    roleblock,
    shoot,
    truesight,
    wolfvote,
    leave,
    stats,
    create,
    frame,
    givepicture,
    heal,
    kill,
    pass,
    possess,
    start_game,
    stop,
    join,
    ls,
    protect,
    frame_and_kill
)]
pub struct Lupus;
