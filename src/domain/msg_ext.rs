use serenity::{
    async_trait,
    model::{
        channel::Message,
        id::{GuildId, UserId},
    },
};

#[async_trait]
pub trait MessageExt {
    fn get_ids(&self) -> (UserId, GuildId);
}

#[async_trait]
impl MessageExt for Message {
    fn get_ids(&self) -> (UserId, GuildId) {
        (self.author.id, self.guild_id.unwrap())
    }
}
