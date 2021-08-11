use std::io::Cursor;
use serenity::{
  prelude::*,
  model::{
      channel::Message,
  },
};
use crate::{
  models::dino::Dino,
  models::teleport::Teleport,
  entities::player::Player,
  internal::*,
  FtpPool,
  DbPool,
  services::{
    message::MessageResponder,
    unbelievabot::*,
  }
};

async fn get_player_object() {

}
