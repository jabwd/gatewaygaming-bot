use crate::models::user::User;
use crate::DbPool;
use serenity::{
    prelude::*,
    model::{
        channel::Message,
        id::UserId,
    },
};

mod dino_options;

pub async fn get_dinobot_context(ctx: &Context, msg: &Message) {
  
}

pub async fn get_message_user(ctx: &Context, msg: &Message) -> User {
  let data = ctx.data.read().await;
  let db = data.get::<DbPool>().unwrap();
  
  User::get(msg.author.id, &db)
}

pub async fn get_user_for_id(ctx: &Context, author_id: UserId) -> User {
  let data = ctx.data.read().await;
  let db = data.get::<DbPool>().unwrap();
  
  User::get(author_id, &db)
}

pub async fn get_db_user(ctx: &Context, discord_id: serenity::model::id::UserId) -> User {
  let data = ctx.data.read().await;
  let db = data.get::<DbPool>().unwrap();
  let user = User::get(discord_id, &db);
  user
}
