use crate::services::message::MessageResponder;
use crate::models::dino::Dino;
use serenity::{
    prelude::*,
    model::{
        channel::Message,
    },
    framework::standard::{ Args, CommandResult, macros::command },
};
use crate::{
    FtpStreamContainer,
    entities::player::Player,
    internal::*
};

#[command]
#[aliases("slay", "s", "kill")]
#[only_in("guilds")]
pub async fn slay(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
  let responder = MessageResponder {
    ctx,
    msg,
  };

  let guild_id = match msg.guild_id {
    Some(guild_id_v) => guild_id_v.0,
    None => {
      return Ok(());
    }
  };

  let user = get_message_user(&ctx, &msg).await;
  let steam_id = match user.get_steam_id() {
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


  let mut read_cursor = match ftp_stream.simple_retr(&file_name).await {
    Ok(cursor) => cursor,
    Err(_) => {
      responder.error("Player not found", "Please make sure you safe logged with a previous dino before attempting an injection").await;
      return Ok(());
    }
  };
  let mut player_object: Player = serde_json::from_reader(&mut read_cursor).unwrap();
  let previous_dino = Dino::game_identifier_to_display_name(&player_object.character_class);

  ftp_stream.rm(&file_name).await.expect("Unable to delete file");

  responder.success("Slay succeeded", format!("Your {} was slain, hf with spawning at murky again :D", previous_dino)).await;

  Ok(())
}