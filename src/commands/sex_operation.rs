use crate::services::message::MessageResponder;
use crate::models::dino::Dino;
use crate::services::unbelievabot::Unbelievabot;
use serenity::{
    prelude::*,
    model::{
        channel::Message,
    },
    framework::standard::{ CommandResult, macros::command },
};
use std::io::Cursor;
use crate::{
    FtpPool,
    entities::player::Player,
    internal::*
};

#[command]
#[aliases("sex-change", "sex-operation")]
#[only_in("guilds")]
async fn sex_change(ctx: &Context, msg: &Message) -> CommandResult {
  let cost: i64 = 1000;
  let responder = MessageResponder {
    ctx,
    msg,
  };
  let guild_id = match msg.guild_id {
    Some(guild_id_unwrapped) => guild_id_unwrapped.0,
    None => {
      return Ok(());
    }
  };

  let user = get_message_user(&ctx, &msg).await;
  let steam_id = match user.get_steam_id() {
    Some(id) => id,
    None => {
      responder.error("No SteamID linked", crate::STEAM_ID_MESSAGE).await;
      return Ok(());
    }
  };

  let balance = Unbelievabot::check_balance(guild_id, msg.author.id.0).await.expect("Unable to fetch balance");
  if balance.cash < cost {
      responder.error("Not enough points", "You do not have enough points to change the sex of your dino").await;
      return Ok(());
  }
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
  let mut player_object: Player = serde_json::from_reader(&mut read_cursor).unwrap();
  let previous_dino = Dino::game_identifier_to_display_name(&player_object.character_class);
  player_object.gender = !player_object.gender;
  let player_file_pretty_str = serde_json::to_string_pretty(&player_object).unwrap();
  let mut reader = Cursor::new(player_file_pretty_str.as_bytes());
  let user_balance = Unbelievabot::remove_cash(guild_id, msg.author.id.0, cost, 0).await.expect("Unable to remove cash");
  ftp_stream.put(&file_name, &mut reader).await.unwrap();

  let gender_str: &str = match player_object.gender {
    true => "female",
    false => "male",
  };
  let replace_message = format!("Your {}'s operation was successful and should now be {}", previous_dino, gender_str);
  responder.respond_injection(
    "Dino injected",
    &replace_message,
    user_balance.cash,
    user_balance.bank,
    cost,
  ).await;

  Ok(())
}
