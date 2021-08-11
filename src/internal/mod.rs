use async_trait::async_trait;

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

#[async_trait]
pub trait MessageUtilities {
  async fn get_user(&self, ctx: &Context) -> User;
}

#[async_trait]
impl MessageUtilities for Message {
  async fn get_user(&self, ctx: &Context) -> User {
    let data = ctx.data.read().await;
    let db = data.get::<DbPool>().unwrap();

    User::get(self.author.id, &db)
  }
}

pub fn gender_from_str(str: &str) -> bool {
  match str {
    "m" => false,
    "male" => false,
    "f" => true,
    "female" => true,
    "fem" => true,
    _ => false,
  }
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
