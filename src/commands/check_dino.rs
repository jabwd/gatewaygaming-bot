use crate::services::message::MessageResponder;
use serenity::{
    prelude::*,
    model::{
        channel::Message,
    },
    framework::standard::{ CommandResult, macros::command },
};
use crate::{
    FtpPool,
    entities::player::Player,
    internal::*
};

#[command]
#[aliases("cd", "checkdino", "healthreport", "hr")]
#[only_in("guilds")]
pub async fn check_dino(ctx: &Context, msg: &Message) -> CommandResult {
  let responder = MessageResponder {
    ctx,
    msg,
  };

  let user = get_message_user(&ctx, &msg).await;
  let steam_id = match user.steam_id {
    Some(id) => id,
    None => {
      responder.error("No SteamID linked", "Link your SteamID first before injecting dinos using the register command").await;
      return Ok(());
    }
  };

  let file_name = format!("{}.json", steam_id);
  let data_read = ctx.data.read().await;
  let pool = data_read.get::<FtpPool>().expect("Expected ftp stream").clone();
  let mut ftp_stream = pool.get().await.expect("Expected FTP connection");
  let mut read_cursor = ftp_stream.simple_retr(&file_name).await.unwrap();
  let player_object: Player = serde_json::from_reader(&mut read_cursor).unwrap();

  Ok(())
}
