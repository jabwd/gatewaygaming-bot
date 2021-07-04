use crate::FtpPool;
use crate::services::message::MessageResponder;
use crate::services::unbelievabot::*;
use crate::models::dino::Dino;
use crate::DbPool;
use serenity::{
    prelude::*,
    model::{
        channel::Message,
    },
    framework::standard::{ Args, CommandResult, macros::command },
};
use std::io::Cursor;
use crate::{
    models::user::*,
    entities::player::Player,
    internal::*
};

#[command]
#[aliases("reg")]
#[only_in("guilds")]
async fn register(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
  let responder = MessageResponder {
    ctx,
    msg,
  };

  let steam_id = match args.single::<String>() {
    Ok(steam_id) => steam_id,
    Err(_) => "".to_string(),
  };
  if steam_id.len() != 17 {
    responder.error("Invalid steamID", "Please fill in a Steam64 ID (xxxxxxxxxxxxxxxxx)").await;
    return Ok(());
  }

  let data = ctx.data.read().await;
  {
    let db = data.get::<DbPool>().unwrap();
    let user = User::get(msg.author.id, &db);

    User::update_steam_id(user.id, &db, &steam_id);
  }

  msg.delete(&ctx).await.expect("Unable to delete sensitive message content");
  responder.success_norply("Steam ID saved", "Your SteamID was registered, and you can now inject dino's").await;
  Ok(())
}

#[command]
#[aliases("dr", "dinorequest")]
#[only_in("guilds")]
async fn dino_request(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
  let responder = MessageResponder {
    ctx,
    msg,
  };

  let dino_key_str = match args.single::<String>() {
    Ok(dino_str) => dino_str,
    Err(_) => "".to_string(),
  };

  if dino_key_str.len() == 0 {
    responder.error("Dino not found", "try one of these: alberto|acro|bary|stego|anky|austro|herrera").await;
    return Ok(());
  }

  let list = Dino::request_dino_list();
  let mut dino_object: Option<&Dino> = None;
  for dino in list.iter() {
    for key in dino.aliases.iter() {
      if key == &dino_key_str {
        dino_object = Some(dino);
        break;
      }
    }
  }

  let user = get_message_user(&ctx, &msg).await;
  let steam_id = match user.steam_id {
    Some(id) => id,
    None => {
      responder.error("No SteamID linked", "Link your SteamID first before injecting dinos using gg.register steamID").await;
      return Ok(());
    }
  };
  if steam_id.len() != 17 {
    responder.error("No SteamID linked", "Link your SteamID first before injecting dinos using gg.register steamID").await;
    return Ok(());
  }

  let dino = match dino_object {
    Some(d) => d,
    None => {
      responder.error("Dino not found", "try one of these: alberto|acro|bary|stego|anky|austro|herrera").await;
      return Ok(());
    }
  };

  if dino.enabled == false {
    responder.error("Dinosaur not available", "That dinosaur is currently not available for injection").await;
    return Ok(());
  }

  let gender = false;

  let guild_id = msg.guild_id.unwrap().0;
  let balance = Unbelievabot::check_balance(guild_id, msg.author.id.0).await.expect("Unable to fetch balance");
  if balance.cash < dino.cost {
      responder.error("Not enough points", "You do not have enough cash points to inject that dino").await;
      return Ok(());
  }

  let data_read = ctx.data.read().await;
  let pool = data_read.get::<FtpPool>().expect("Expected ftp stream").clone();
  let mut ftp_stream = pool.get().await.expect("Expected FTP connection");

  let file_name = format!("{}.json", steam_id);
  
  let mut read_cursor = ftp_stream.simple_retr(&file_name).await.unwrap();
  let mut player_object: Player = serde_json::from_reader(&mut read_cursor).unwrap();
  let previous_dino = player_object.character_class.to_string();
  player_object.update_from_dino(&dino, gender);
  let player_file_pretty_str = serde_json::to_string_pretty(&player_object).unwrap();
  let mut reader = Cursor::new(player_file_pretty_str.as_bytes());
  ftp_stream.put(&file_name, &mut reader).await.unwrap();
  // let user_balance = Unbelievabot::remove_cash(guild_id, msg.author.id.0, dino.cost, 0).await.expect("Unable to remove cash");
  let replace_message = format!("Your {} was replaced with an injected {}", previous_dino, dino.display_name);
  responder.respond_injection(
    "Dino injected",
    &replace_message,
    balance.cash,
    balance.bank,
    dino.cost,
  ).await;

  Ok(())
}

#[command]
#[aliases("cashbuy", "cb")]
#[only_in("guilds")]
async fn cash_buy(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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

  let yesno = match args.single::<String>() {
    Ok(yesno) => Some(yesno),
    Err(_) => None,
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

  let progress_msg = responder.in_progress("Injecting dino", "Contemplating whether to leg break you…").await.expect("Expected a progress msg");
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
  player_object.update_from_dino(&dino, gender);
  let player_file_pretty_str = serde_json::to_string_pretty(&player_object).unwrap();
  let mut reader = Cursor::new(player_file_pretty_str.as_bytes());
  let user_balance = Unbelievabot::remove_cash(guild_id, msg.author.id.0, dino.cost, 0).await.expect("Unable to remove cash");
  ftp_stream.put(&file_name, &mut reader).await.unwrap();

  let _ = progress_msg.delete(&ctx).await;

  let replace_message = format!("Your {} was replaced with an injected {}", previous_dino, dino.display_name);
  responder.respond_injection(
    "Dino injected",
    &replace_message,
    user_balance.cash,
    user_balance.bank,
    dino.cost,
  ).await;

  if let Some(yesno) = yesno {
    if yesno == "y" || yesno == "n" {
      responder.success("", "P.S. I don't need that y/n thing on the end of the command :)").await;
    }
  }

  Ok(())
}

#[command]
#[aliases("inject", "i")]
#[only_in("guilds")]
async fn admin_inject(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
  let responder = MessageResponder {
    ctx,
    msg,
  };

  let guild_id = msg.guild_id.unwrap().0;
  let guild = msg.guild(&ctx).await.unwrap();
  let mut is_authorised = false;
  for (role_id, role) in guild.roles {
    if role.name.contains("Admins") {
      let has_role = match msg.author.has_role(&ctx, guild_id, role_id).await {
        Ok(user_has_role) => user_has_role,
        Err(_) => false,
      };
      is_authorised = has_role;
      break;
    }
  }
  if !is_authorised {
    responder.error(":o", "I don't think you're allowed to touch this command").await;
    return Ok(());
  }

  if msg.mentions.len() < 1 {
    responder.error("No user mentioned", "Please mention the user to forcefully inject, gg.i @user dino gender").await;
    return Ok(());
  }
  let target_user = &msg.mentions[0];

  let _ = args.single::<String>();
  let dino_key_str = match args.single::<String>() {
    Ok(dino_str) => dino_str,
    Err(_) => "".to_string(),
  };

  let gender_str = match args.single::<String>() {
      Ok(gender_str) => gender_str,
      Err(_) => "".to_string(),
  };

  if dino_key_str.len() == 0 {
    msg.reply(&ctx, "Usage: gg.i @user dino gender").await.expect("Unable to reply to message");
    return Ok(());
  }

  if gender_str.len() == 0 {
    msg.reply(&ctx, "Usage: gg.i @user dino gender").await.expect("Unable to reply to message");
    return Ok(());
  }

  let list = Dino::list();
  let mut dino_object: Option<&Dino> = None;
  for dino in list.iter() {
    for key in dino.aliases.iter() {
      if key == &dino_key_str {
        dino_object = Some(dino);
        break;
      }
    }
  }

  let user = get_db_user(&ctx, target_user.id).await;
  let steam_id = match user.steam_id {
    Some(id) => id,
    None => {
      responder.error("No SteamID linked", "Link your SteamID first before injecting dinos using gg.register steamID").await;
      return Ok(());
    }
  };
  if steam_id.len() != 17 {
    responder.error("No SteamID linked", "Link your SteamID first before injecting dinos using gg.register steamID").await;
    return Ok(());
  }

  let dino = match dino_object {
    Some(d) => d,
    None => {
      responder.error("Dino not found", "Consult staff for all the available options for admin injection").await;
      return Ok(());
    }
  };

  let gender = match gender_str.as_str() {
      "m" => false,
      "male" => false,
      "f" => true,
      "female" => true,
      "fem" => true,
      _ => {
          msg.reply(&ctx, "Usage: gg.cb dinosaur m|f").await.expect("Unable to reply to message");
          return Ok(());
      },
  };

  let data_read = ctx.data.read().await;
  let pool = data_read.get::<FtpPool>().expect("Expected ftp stream").clone();
  let mut ftp_stream = pool.get().await.expect("Expected FTP connection");

  let file_name = format!("{}.json", steam_id);
  let file_list = ftp_stream.nlst(None).await.unwrap();

  let mut found = false;
  for file in file_list {
    if file == file_name {
      found = true;
      let mut read_cursor = ftp_stream.simple_retr(&file).await.unwrap();
      let mut player_object: Player = serde_json::from_reader(&mut read_cursor).unwrap();
      let previous_dino = Dino::game_identifier_to_display_name(&player_object.character_class);
      player_object.update_from_dino(&dino, gender);
      let player_file_pretty_str = serde_json::to_string_pretty(&player_object).unwrap();
      let mut reader = Cursor::new(player_file_pretty_str.as_bytes());
      ftp_stream.put(&file_name, &mut reader).await.unwrap();
      let replace_message = format!("{}'s {} was replaced with an injected {}", target_user.name, previous_dino, dino.display_name);
      responder.respond_admin_injection(
        "Dino injected",
        &replace_message,
        &target_user,
      ).await;
      break;
    }
  }

  if found == false {
    responder.error("Player not found", "Please make sure you log in with a random dinosaur on the server first before injecting anything.").await;
  }
  Ok(())
}

#[command]
#[aliases("randomdino", "rd")]
#[only_in("guilds")]
async fn random_dino(ctx: &Context, msg: &Message) -> CommandResult {
  let responder = MessageResponder {
    ctx,
    msg,
  };

  responder.error("Not avialable", "Random dino is not yet available").await;
  Ok(())
}
