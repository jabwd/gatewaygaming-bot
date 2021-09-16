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
pub trait ContextUtilities {
  async fn get_user(&self, id: UserId) -> User;
}

#[async_trait]
impl ContextUtilities for Context {
  async fn get_user(&self, id: UserId) -> User {
    let data = self.data.read().await;
    let db = data.get::<DbPool>().unwrap();
    let user = User::get(id, &db);
    user
  }
}

#[async_trait]
pub trait MessageUtilities {
  async fn get_user(&self, ctx: &Context) -> User;
}

#[async_trait]
impl MessageUtilities for Message {
  async fn get_user(&self, ctx: &Context) -> User {
    ctx.get_user(self.author.id).await
  }
}

macro_rules! unwrap_or_return {
  ( $e:expr ) => {
    match $e {
      Ok(x) => x,
      Err(_) => Ok(())
    }
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
