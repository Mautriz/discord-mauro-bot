pub mod frame;
pub mod givepicture;
pub mod heal;
pub mod kill;
pub mod pass;
pub mod possess;
pub mod protect;
pub mod roleblock;
pub mod shoot;
pub mod start_vote;
pub mod truesight;
pub mod wolfvote;

// use frame::*;
// use givepicture::*;
// use heal::*;
// use kill::*;
// use pass::*;
// use possess::*;
// use protect::*;
// use roleblock::*;
// use shoot::*;
// use start_vote::*;
// use truesight::*;
// use wolfvote::*;

// #[command]
// #[sub_commands(
//     roleblock,
//     frame,
//     givepicture,
//     protect,
//     kill,
//     wolfvote,
//     truesight,
//     possess,
//     start_vote,
//     heal,
//     shoot,
//     pass
// )]
// pub async fn action(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
//     let _ = msg
//         .channel_id
//         .say(&ctx.http, format!("Please specify an action"))
//         .await;
//     Ok(())
// }
