use std::io::Cursor;
use chrono::prelude::*;
use serenity::{framework::standard::{ Args, CommandResult, macros::command }, model::{channel::{Message, Reaction}}, prelude::*};
use crate::{
    FtpPool,
    DbPool,
    entities::player::Player,
    internal::*,
    contexts::garage::*,
    services::message::MessageResponder,
    models::{dino::Dino, garage::Garage, garage::GarageSlotInsertable}
};

#[command]
#[aliases("exterminate", "clear")]
#[only_in("guilds")]
async fn exterminate_garage(ctx: &Context, msg: &Message) -> CommandResult {
  let responder = MessageResponder {
    ctx,
    msg,
  };

  let user = msg.get_user(&ctx).await;
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
    responder.success("Dino garage", "Your dino garage appears to be empty. You cannot kill those who do not yet exist.").await;
    return Ok(());
  }

  Garage::clear_for_user(user.id, &db);

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

  let user = msg.get_user(&ctx).await;
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
  let sent_msg = match responder.dino_garage(&slots, false).await {
    Ok(msg) => msg,
    Err(why) => {
      println!("Unable to post garage list {}", why);
      return Ok(());
    }
  };

  let reaction_context_lock = {
    let data = ctx.data.read().await;

    data.get::<crate::ReactionContext>().expect("Unable to find reaction context").clone()
  };

  {
    let context = GarageResponseReactionContext
    {
      msg_id: sent_msg.id.0,
      author_id: msg.author.id.0,
      slots: slots.into_iter().map(|x| x).collect(),
      response_type: GarageResponseType::List,
      created: Utc::now(),
    };
    let mut reaction_context = reaction_context_lock.write().await;

    reaction_context.garage_context_list.push(context);
  }

  Ok(())
}

pub async fn garage_handle_list_reaction(ctx: &Context, msg: &Message, reaction: &Reaction, garage_ctx: &GarageResponseReactionContext) -> CommandResult {
  let referenced_message = match &msg.referenced_message {
    Some(msg) => msg,
    None => {
      println!("No ref message");
      return Ok(());
    }
  };
  let responder = MessageResponder {
    ctx,
    msg: referenced_message,
  };

  let discord_user = reaction.user(&ctx).await.unwrap();
  let user = ctx.get_user(discord_user.id).await;
  let steam_id = match user.get_steam_id() {
    Some(id) => id,
    None => {
      responder.error("No SteamID linked", crate::STEAM_ID_MESSAGE).await;
      return Ok(());
    }
  };

  // Determine which slot we wanted according to the emoji number
  let idx: Option<usize> = match reaction.emoji.to_string().as_str() {
    "❌" => None,
    "1️⃣" => Some(0),
    "2️⃣" => Some(1),
    "3️⃣" => Some(2),
    "4️⃣" => Some(3),
    "5️⃣" => Some(4),
    _ => None,
  };

  let idx = match idx {
    Some(idx ) => idx,
    None => {
      println!("No index found");
      let _ = msg.delete(&ctx).await;
      return Ok(());
    }
  };

  let selected_slot = match garage_ctx.slots.as_slice().get(idx) {
    Some(slot) => slot,
    None => {
      return Ok(());
    }
  };

  let player = selected_slot.player();
  let file_name = format!("{}.json", steam_id);
  let data_read = ctx.data.read().await;
  let db = data_read.get::<DbPool>().unwrap();
  let pool = data_read.get::<FtpPool>().expect("Expected ftp stream").clone();
  let mut ftp_stream = pool.get().await.expect("Expected FTP connection");

  let player_file_pretty_str = serde_json::to_string_pretty(&player).unwrap();
  let mut reader = Cursor::new(player_file_pretty_str.as_bytes());
  ftp_stream.put(&file_name, &mut reader).await.unwrap();
  Garage::delete_slot(selected_slot.id, &db);
  let _ = msg.delete(&ctx).await;
  responder.success("Dino deployed", &format!("Your {} was released onto the island. Go butter some crumpets", Dino::game_identifier_to_display_name(&player.character_class))).await;
  Ok(())
}

pub async fn garage_handle_swap_dino(ctx: &Context, msg: &Message, reaction: &Reaction, garage_ctx: &GarageResponseReactionContext) -> CommandResult {
  let referenced_message = match &msg.referenced_message {
    Some(msg) => msg,
    None => {
      println!("No ref message");
      return Ok(());
    }
  };
  let responder = MessageResponder {
    ctx,
    msg: referenced_message,
  };

  let discord_user = reaction.user(&ctx).await.unwrap();
  let user = ctx.get_user(discord_user.id).await;
  let steam_id = match user.get_steam_id() {
    Some(id) => id,
    None => {
      responder.error("No SteamID linked", crate::STEAM_ID_MESSAGE).await;
      return Ok(());
    }
  };

  // Determine which slot we wanted according to the emoji number
  let idx: Option<usize> = match reaction.emoji.to_string().as_str() {
    "❌" => None,
    "1️⃣" => Some(0),
    "2️⃣" => Some(1),
    "3️⃣" => Some(2),
    "4️⃣" => Some(3),
    "5️⃣" => Some(4),
    _ => None,
  };

  let idx = match idx {
    Some(idx ) => idx,
    None => {
      println!("No index found");
      let _ = msg.delete(&ctx).await;
      return Ok(());
    }
  };

  let selected_slot = match garage_ctx.slots.as_slice().get(idx) {
    Some(slot) => slot,
    None => {
      return Ok(());
    }
  };

  let player = selected_slot.player();
  let file_name = format!("{}.json", steam_id);
  let data_read = ctx.data.read().await;
  let db = data_read.get::<DbPool>().unwrap();
  let pool = data_read.get::<FtpPool>().expect("Expected ftp stream").clone();
  let mut ftp_stream = pool.get().await.expect("Expected FTP connection");
  let mut read_cursor = match ftp_stream.simple_retr(&file_name).await {
    Ok(cursor) => cursor,
    Err(_) => {
      responder.error("Player not found", "Please make sure you safe logged with a previous dino before attempting a swap").await;
      return Ok(());
    }
  };

  let player_object: Player = serde_json::from_reader(&mut read_cursor).unwrap();
  let saved_dinosaur_name = Dino::game_identifier_to_display_name(&player_object.character_class);

  let player_file_pretty_str = serde_json::to_string_pretty(&player).unwrap();
  let mut reader = Cursor::new(player_file_pretty_str.as_bytes());
  ftp_stream.put(&file_name, &mut reader).await.unwrap();
  Garage::delete_slot(selected_slot.id, &db);

  let save_name = "".to_string();
  let new_slot = GarageSlotInsertable::from_player_object(&player_object, user.id, &save_name);
  Garage::save_slot(&new_slot, &db);
  let _ = msg.delete(&ctx).await;
  responder.success("Dino deployed", &format!("Your {} was swapped for your {}. Go butter some crumpets", saved_dinosaur_name, Dino::game_identifier_to_display_name(&player.character_class))).await;
  Ok(())
}

pub async fn garage_handle_delete_reaction(ctx: &Context, msg: &Message, reaction: &Reaction, garage_ctx: &GarageResponseReactionContext) -> CommandResult {
  let referenced_message = match &msg.referenced_message {
    Some(msg) => msg,
    None => {
      println!("No ref message");
      return Ok(());
    }
  };
  let responder = MessageResponder {
    ctx,
    msg: referenced_message,
  };

  // Determine which slot we wanted according to the emoji number
  let idx: Option<usize> = match reaction.emoji.to_string().as_str() {
    "❌" => None,
    "1️⃣" => Some(0),
    "2️⃣" => Some(1),
    "3️⃣" => Some(2),
    "4️⃣" => Some(3),
    "5️⃣" => Some(4),
    _ => None,
  };

  let idx = match idx {
    Some(idx ) => idx,
    None => {
      println!("No index found");
      let _ = msg.delete(&ctx).await;
      return Ok(());
    }
  };

  let selected_slot = match garage_ctx.slots.as_slice().get(idx) {
    Some(slot) => slot,
    None => {
      return Ok(());
    }
  };

  let data_read = ctx.data.read().await;
  let db = data_read.get::<DbPool>().unwrap();
  Garage::delete_slot(selected_slot.id, &db);

  let _ = msg.delete(&ctx).await;
  responder.success(
    "Dino taken out back", 
    &format!("Your {} was taken out back and will no longer return to your garage. Ignore the gun shot noises, they are not related.", Dino::game_identifier_to_display_name(&selected_slot.character_class))
  ).await;
  Ok(())
}

#[command]
#[aliases("swap")]
#[only_in("guilds")]
async fn garage_swap_dino(ctx: &Context, msg: &Message) -> CommandResult {
  let responder = MessageResponder {
    ctx,
    msg,
  };

  let user = msg.get_user(&ctx).await;
  let steam_id = match user.get_steam_id() {
    Some(id) => id,
    None => {
      responder.error("No SteamID linked", crate::STEAM_ID_MESSAGE).await;
      return Ok(());
    }
  };

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

  // Detect what dino we are trying to swap out
  let pool = data.get::<FtpPool>().expect("Expected ftp stream").clone();
  let mut ftp_stream = pool.get().await.expect("Expected FTP connection");
  let file_name = format!("{}.json", steam_id);

  let mut read_cursor = match ftp_stream.simple_retr(&file_name).await {
    Ok(cursor) => cursor,
    Err(_) => {
      responder.error("Player not found", "Please make sure you safe logged with a previous dino before attempting an injection").await;
      return Ok(());
    }
  };

  let player_object: Player = serde_json::from_reader(&mut read_cursor).unwrap();
  let saved_dinosaur_name = Dino::game_identifier_to_display_name(&player_object.character_class);

  let sent_msg = match responder.dino_garage_swap(&slots, &saved_dinosaur_name).await {
    Ok(msg) => msg,
    Err(why) => {
      println!("Unable to post garage list {}", why);
      return Ok(());
    }
  };

  let reaction_context_lock = {
    let data = ctx.data.read().await;

    data.get::<crate::ReactionContext>().expect("Unable to find reaction context").clone()
  };

  {
    let context = GarageResponseReactionContext
    {
      msg_id: sent_msg.id.0,
      author_id: msg.author.id.0,
      slots: slots.into_iter().map(|x| x).collect(),
      response_type: GarageResponseType::Swap,
      created: Utc::now(),
    };
    let mut reaction_context = reaction_context_lock.write().await;

    reaction_context.garage_context_list.push(context);
  }

  Ok(())
}

#[command]
#[aliases("delete")]
#[only_in("guilds")]
async fn garage_delete(ctx: &Context, msg: &Message) -> CommandResult {
  let responder = MessageResponder {
    ctx,
    msg,
  };

  let user = msg.get_user(&ctx).await;
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
  let sent_msg = match responder.dino_garage(&slots, true).await {
    Ok(msg) => msg,
    Err(why) => {
      println!("Unable to post garage list {}", why);
      return Ok(());
    }
  };

  let reaction_context_lock = {
    let data = ctx.data.read().await;

    data.get::<crate::ReactionContext>().expect("Unable to find reaction context").clone()
  };

  {
    let context = GarageResponseReactionContext
    {
      msg_id: sent_msg.id.0,
      author_id: msg.author.id.0,
      slots: slots.into_iter().map(|x| x).collect(),
      response_type: GarageResponseType::Delete,
      created: Utc::now(),
    };
    let mut reaction_context = reaction_context_lock.write().await;

    reaction_context.garage_context_list.push(context);
  }

  Ok(())
}

#[command]
#[aliases("restore", "restore-dino", "rd")]
#[only_in("guilds")]
async fn restore_garage_slot(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
  let responder = MessageResponder {
    ctx,
    msg,
  };
  let save_name = "".to_string();

  let user = msg.get_user(&ctx).await;
  let steam_id = match user.get_steam_id() {
    Some(id) => id,
    None => {
      responder.error("No SteamID linked", "Link your SteamID first before injecting dinos using register command").await;
      return Ok(());
    }
  };
  let file_name = format!("{}.json", steam_id);

  let data = ctx.data.read().await;
  let db = data.get::<DbPool>().unwrap();
  let slots = match Garage::slots_for_user(user.id, &db) {
    Some(list) => list,
    None => {
      println!("No list found");
      return Ok(());
    }
  };

  if slots.len() >= 4 {
    responder.error("Garage full", "Your dino garage is full, storing more dinosaurs would be unethical").await;
    return Ok(());
  }

  let mut selected_slot: Option<Garage> = None;
  for slot in slots {
    if slot.slot_name.to_lowercase().contains(save_name.to_lowercase().as_str()) {
      selected_slot = Some(slot);
      break;
    }
  }
  let selected_slot = match selected_slot {
    Some(slot) => slot,
    None => {
      responder.error("Not found", "Slot not found").await;
      return Ok(());
    }
  };
  let player = selected_slot.player();

  
  let data_read = ctx.data.read().await;
  let pool = data_read.get::<FtpPool>().expect("Expected ftp stream").clone();
  let mut ftp_stream = pool.get().await.expect("Expected FTP connection");

  let player_file_pretty_str = serde_json::to_string_pretty(&player).unwrap();
  let mut reader = Cursor::new(player_file_pretty_str.as_bytes());
  ftp_stream.put(&file_name, &mut reader).await.unwrap();
  Garage::delete_slot(selected_slot.id, &db);
  responder.success("Dino deployed", &format!("Your {} was released onto the island. Go butter some crumpets", Dino::game_identifier_to_display_name(&player.character_class))).await;

  Ok(())
}

#[command]
#[aliases("save", "save-dino", "s", "sd")]
#[only_in("guilds")]
async fn garage_save_dino(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
  let responder = MessageResponder {
    ctx,
    msg,
  };

  let save_name = "na".to_string();

  let user = msg.get_user(&ctx).await;
  let steam_id = match user.get_steam_id() {
    Some(id) => id,
    None => {
      responder.error("No SteamID linked", crate::STEAM_ID_MESSAGE).await;
      return Ok(());
    }
  };

  let file_name = format!("{}.json", steam_id);
  let data_read = ctx.data.read().await;

  let db = data_read.get::<DbPool>().unwrap();
  let slots = match Garage::slots_for_user(user.id, &db) {
    Some(list) => list,
    None => {
      println!("No list found");
      return Ok(());
    }
  };

  if slots.len() >= 2 {
    responder.error("Garage full", "Your dino garage is full, storing more dinosaurs would be unethical").await;
    return Ok(());
  }

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

  responder.success("Dino stored in your garage", &format!("Your {} was stored in your garage", saved_dinosaur_name)).await;

  Ok(())
}
