use crate::models::user::User;
use crate::DbPool;
use serenity::{
    prelude::*,
    model::{
        channel::Message,
    },
};

mod dino_options;
mod ftp_stream_manager;

pub async fn get_message_user(ctx: &Context, msg: &Message) -> User {
    let data = ctx.data.read().await;
    let db = data.get::<DbPool>().unwrap();
    let user = User::get(msg.author.id, &db);
    user
}
