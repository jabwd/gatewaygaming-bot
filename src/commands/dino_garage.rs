use crate::DbPool;
use crate::models::garage::Garage;
use crate::models::garage::GarageSlotInsertable;
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
    FtpPool,
    entities::player::Player,
    internal::*
};

#[command]
#[aliases("exterminate", "clear")]
#[only_in("guilds")]
async fn exterminate_garage(ctx: &Context, msg: &Message) -> CommandResult {
  Ok(())
}

#[command]
#[aliases("garage", "list")]
#[only_in("guilds")]
async fn garage_list(ctx: &Context, msg: &Message) -> CommandResult {
  let responder = MessageResponder {
    ctx,
    msg,
  };

  let user = get_message_user(&ctx, &msg).await;

  let data = ctx.data.read().await;
  let db = data.get::<DbPool>().unwrap();
  let slots = match Garage::slots_for_user(user.id, &db) {
    Some(list) => list,
    None => {
      println!("No list found");
      return Ok(());
    }
  };

  if slots.len() == 0 {
    responder.success("Dino garage", "Your dino garage appears to be empty. Grow some dinosaurs, or inject them idk. Do things that give you dino.").await;
    return Ok(());
  }

  let mut list_str = String::new();
  let mut count = 1;
  for slot in slots {
    let dino_name = Dino::game_identifier_to_display_name(&slot.character_class);
    list_str.push_str(&format!("{}.) {}:\t {}\n", count, slot.slot_name, dino_name));
    count += 1;
  }
  responder.success("Your dino garage:", &list_str).await;

  Ok(())
}

#[command]
#[aliases("save", "save-dino", "s", "sd")]
#[only_in("guilds")]
async fn garage_save_dino(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
  let responder = MessageResponder {
    ctx,
    msg,
  };

  let save_name = match args.single::<String>() {
    Ok(save_key) => save_key,
    Err(_) => {
      responder.cb_usage().await;
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

  let guild_id = msg.guild_id.unwrap().0;

  let file_name = format!("{}.json", steam_id);
  let data_read = ctx.data.read().await;
  let pool = data_read.get::<FtpPool>().expect("Expected ftp stream").clone();
  let mut ftp_stream = pool.get().await.expect("Expected FTP connection");

  let mut read_cursor = match ftp_stream.simple_retr(&file_name).await {
    Ok(cursor) => cursor,
    Err(_) => {
      responder.error("Player not found", "Please make sure you safe logged with a previous dino before attempting an injection").await;
      return Ok(());
    }
  };

  let player_object: Player = serde_json::from_reader(&mut read_cursor).unwrap();
  let saved_dinosaur_name = Dino::game_identifier_to_display_name(&player_object.character_class);

  {
    let data = ctx.data.read().await;
    let db = data.get::<DbPool>().unwrap();

    let new_slot = GarageSlotInsertable::from_player_object(&player_object, user.id, &save_name);
    Garage::save_slot(&new_slot, &db);
    ftp_stream.rm(&file_name).await.expect("Unable to delete dino file");
  }

  responder.success("Dino stored in your garage", &format!("Your {} was stored in your garage with the name tag {}", saved_dinosaur_name, &save_name)).await;

  Ok(())
}
