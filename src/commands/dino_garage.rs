use crate::services::message::MessageResponder;
use crate::services::unbelievabot::*;
use crate::models::dino::Dino;
use serenity::{
    prelude::*,
    model::{
        channel::Message,
    },
    framework::standard::{ Args, CommandResult, macros::command },
};
use std::io::Cursor;
use crate::{
    FtpStreamContainer,
    entities::player::Player,
    internal::*
};

#[command]
#[aliases("garage")]
#[only_in("guilds")]
async fn garage(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
  let responder = MessageResponder {
    ctx,
    msg,
  };

  let dino_key_str = match args.single::<String>() {
    Ok(dino_str) => dino_str,
    Err(_) => {
      responder.cb_usage().await;
      return Ok(());
    }
  };

  let gender_str = match args.single::<String>() {
      Ok(gender_str) => gender_str,
      Err(_) => {
        responder.cb_usage().await;
        return Ok(());
      }
  };

  let list = Dino::list();
  let mut dino_object: Option<&Dino> = None;
  for dino in list.iter() {
    for key in dino.aliases.iter() {
      if key.to_lowercase() == dino_key_str.to_string().to_lowercase() {
        dino_object = Some(dino);
        break;
      }
    }
  }

  let user = get_message_user(&ctx, &msg).await;
  let steam_id = match user.get_steam_id() {
    Some(id) => id,
    None => {
      responder.error("No SteamID linked", "Link your SteamID first before injecting dinos using gg.register steamID").await;
      return Ok(());
    }
  };

  let dino = match dino_object {
    Some(d) => d,
    None => {
      responder.cb_usage().await;
      return Ok(());
    }
  };

  if dino.enabled == false {
    responder.error("Dinosaur not available", "That dinosaur is currently not available for injection.\nSome dinosaurs are disabled due to specific rules about injection,\ncheck the store for more information.").await;
    return Ok(());
  }

  let gender = match gender_str.to_lowercase().as_str() {
      "m" => false,
      "male" => false,
      "f" => true,
      "female" => true,
      "fem" => true,
      _ => {
          responder.cb_usage().await;
          return Ok(());
      },
  };

  let guild_id = msg.guild_id.unwrap().0;
  let balance = Unbelievabot::check_balance(guild_id, msg.author.id.0).await.expect("Unable to fetch balance");
  if balance.cash < dino.cost {
      responder.error("Not enough points", "You do not have enough cash points to inject that dino").await;
      return Ok(());
  }

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
  player_object.update_from_dino(&dino, gender);
  let player_file_pretty_str = serde_json::to_string_pretty(&player_object).unwrap();
  let mut reader = Cursor::new(player_file_pretty_str.as_bytes());
  let user_balance = Unbelievabot::remove_cash(guild_id, msg.author.id.0, dino.cost, 0).await.expect("Unable to remove cash");
  ftp_stream.put(&file_name, &mut reader).await.unwrap();

  let replace_message = format!("Your {} was replaced with an injected {}", previous_dino, dino.display_name);
  responder.respond_injection(
    "Dino injected",
    &replace_message,
    user_balance.cash,
    user_balance.bank,
    dino.cost,
  ).await;

  Ok(())
}
