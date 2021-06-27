use crate::services::message::MessageResponder;
use serenity::{
    prelude::*,
    model::{
        channel::Message,
    },
    framework::standard::{ CommandResult, macros::command },
};
use crate::{
    FtpStreamContainer,
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
  let mut read_cursor = ftp_stream.simple_retr(&file_name).await.unwrap();
  let player_object: Player = serde_json::from_reader(&mut read_cursor).unwrap();
  println!("Prev dino: {:?}", player_object);
  println!("Isle location: {:?}\nIsle Rotation: {:?}\nCamera rotation: {:?}\nCamera distance: {:?}", player_object.location_isle_v3, player_object.rotation_isle_v3, player_object.camera_rotation_isle_v3, player_object.camera_distance_isle_v3);

  Ok(())
}
