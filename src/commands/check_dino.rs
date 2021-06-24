use crate::services::message::MessageResponder;
use crate::services::unbelievabot::*;
use serenity::utils::Colour;
use crate::models::dino::Dino;
use crate::DbPool;
use serenity::{
    prelude::*,
    model::{
        channel::Message,
        guild::GuildContainer,
    },
    framework::standard::{ Args, CommandResult, macros::command },
};
use serenity::model::channel::ReactionType;
use std::io::Cursor;
use crate::{
    models::user::*,
    FtpStreamContainer,
    entities::player::Player,
    internal::*
};

#[command]
#[aliases("cd", "checkdino", "healthreport", "hr")]
#[only_in("guilds")]
pub async fn check_dino(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
  let responder = MessageResponder {
    ctx,
    msg,
  };

  let user = get_message_user(&ctx, &msg).await;
  let steam_id = match user.steam_id {
    Some(id) => id,
    None => {
      responder.error("No SteamID linked", "Link your SteamID first before injecting dinos using gg.register steamID").await;
      return Ok(());
    }
  };

  let ftp_stream_lock = {
    let data_read = ctx.data.read().await;
    data_read.get::<FtpStreamContainer>().expect("Expected FTP stream").clone()
  };

  let file_name = format!("{}.json", steam_id);
  let mut ftp_stream = ftp_stream_lock.lock().await;
  let file_list = ftp_stream.nlst(None).await.unwrap();

  let mut found = false;
  for file in file_list {
    if file == file_name {
      found = true;
      let mut read_cursor = ftp_stream.simple_retr(&file).await.unwrap();
      let player_object: Player = serde_json::from_reader(&mut read_cursor).unwrap();
      println!("Prev dino: {:?}", player_object);
      break;
    }
  }

  if found == false {
    responder.error("Player not found", "Please make sure you log in with a random dinosaur on the server first before injecting anything.").await;
  }
  Ok(())
}
