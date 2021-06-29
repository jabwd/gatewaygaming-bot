use crate::services::message::MessageResponder;
use crate::services::unbelievabot::*;
use serenity::{
    prelude::*,
    model::{
        channel::Message,
    },
    framework::standard::{ Args, CommandResult, macros::command },
};
use std::io::Cursor;
use crate::{
    models::dino::Dino,
    models::teleport::Teleport,
    entities::player::Player,
    internal::*,
    FtpPool,
};

#[command]
#[aliases("teleport", "tp")]
#[only_in("guilds")]
pub async fn teleport(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
  let responder = MessageResponder {
    ctx,
    msg,
  };

  // tp to murky: Wisco's favorite spawn
  // Slay: Slay confirmed. Crumpets buttered.

  let guild_id = match msg.guild_id {
    Some(guild_id_v) => guild_id_v.0,
    None => {
      return Ok(());
    }
  };

  let location_label = match args.single::<String>() {
    Ok(loc) => loc,
    Err(_) => {
      responder.tp_usage().await;
      return Ok(());
    }
  };

  let list = Teleport::tp_locations();
  let mut locations = vec![];
  for location in &list {
    if location.label.contains(&location_label) {
      locations.insert(0, location);
    }
  }

  if locations.len() == 0 {
    responder.tp_usage().await;
    return Ok(());
  }

  let user = get_message_user(&ctx, &msg).await;
  let steam_id = match user.get_steam_id() {
    Some(id) => id,
    None => {
      responder.error("No SteamID linked", "Link your SteamID first before injecting dinos using gg.register steamID").await;
      return Ok(());
    }
  };

  let cost = 1000;
  let balance = Unbelievabot::check_balance(guild_id, msg.author.id.0).await.expect("Unable to fetch balance");
  if balance.cash < cost {
      responder.error("Not enough points", "You do not have enough cash points to teleport a dino right now").await;
      return Ok(());
  }

  let data_read = ctx.data.read().await;
  let pool = data_read.get::<FtpPool>().expect("Expected ftp stream").clone();
  let mut ftp_stream = pool.get().await.expect("Expected FTP connection");
  let file_name = format!("{}.json", steam_id);

  let mut read_cursor = match ftp_stream.simple_retr(&file_name).await {
    Ok(cursor) => cursor,
    Err(_) => {
      responder.error("Player not found", "Please make sure you safe logged with a previous dino before attempting an injection").await;
      return Ok(());
    }
  };
  let mut player_object: Player = serde_json::from_reader(&mut read_cursor).unwrap();
  let previous_dino = Dino::game_identifier_to_display_name(&player_object.character_class);

  let rand_index = rand::random::<usize>() % locations.len();

  let selected_location = locations[rand_index];
  player_object.update_teleport(&selected_location);
  let player_file_pretty_str = serde_json::to_string_pretty(&player_object).unwrap();
  let mut reader = Cursor::new(player_file_pretty_str.as_bytes());
  let user_balance = Unbelievabot::remove_cash(guild_id, msg.author.id.0, cost, 0).await.expect("Unable to remove cash");
  ftp_stream.put(&file_name, &mut reader).await.unwrap();
  responder.respond_tp("Teleport done", format!("Teleported your {} to {}", previous_dino, location_label).as_str(), user_balance.cash, user_balance.bank, cost, &msg.author).await;

  Ok(())
}
