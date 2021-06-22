use crate::services::message::MessageResponder;
use crate::services::unbelievabot::*;
use serenity::utils::Colour;
use crate::models::dino::Dino;
use crate::DbPool;
use serenity::{
    prelude::*,
    model::{
        channel::Message,
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
#[aliases("rd", "requestdino")]
#[only_in("guilds")]
async fn request_dino(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
  let responder = MessageResponder {
    ctx,
    msg,
  };

  responder.error("Not implemented", "this command has not been finished yet").await;
  Ok(())
}

#[command]
#[aliases("cashbuy", "cb")]
#[only_in("guilds")]
async fn cash_buy(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // cashbuy rex male
    let dino_key_str = match args.single::<String>() {
        Ok(dino_str) => dino_str,
        Err(_) => "".to_string(),
    };

    let gender_str = match args.single::<String>() {
        Ok(gender_str) => gender_str,
        Err(_) => "".to_string(),
    };

    if dino_key_str.len() == 0 {
        msg.reply(&ctx, "Usage: !cb {dino} {gender}").await.expect("Unable to reply to message");
        return Ok(());
    }

    if gender_str.len() == 0 {
        msg.reply(&ctx, "Usage: !cb {dino} {gender}").await.expect("Unable to reply to message");
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

    let dino = match dino_object {
        Some(d) => d,
        None => {
            msg.reply(&ctx, "Dino not found").await.expect("Unable to reply to message");;
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
            msg.reply(&ctx, "Usage: !cb {dino} m|f").await.expect("Unable to reply to message");
            return Ok(());
        },
    };

    let ftp_stream_lock = {
        let data_read = ctx.data.read().await;
        data_read.get::<FtpStreamContainer>().expect("Expected FTP stream").clone()
    };

    let user = get_message_user(&ctx, &msg).await;
    if let Some(steam_id) = user.steam_id {
        if steam_id.len() != 17 {
            msg.reply(&ctx, "No valid steam ID registered for this user").await.expect("unable to reply to msg");
            return Ok(());
        }
        let mut ftp_stream = ftp_stream_lock.lock().await;

        let file_list = ftp_stream.nlst(None).await.unwrap();
        let file_name = format!("{}.json", steam_id);

        let mut found = false;
        for file in file_list {
            if file == file_name {
                found = true;
                let mut read_cursor = ftp_stream.simple_retr(&file).await.unwrap();
                let mut player_object: Player = serde_json::from_reader(&mut read_cursor).unwrap();
                let previous_dino = player_object.character_class.to_string();
                player_object.update_from_dino(&dino, gender);
                let player_file_pretty_str = serde_json::to_string_pretty(&player_object).unwrap();
                let mut reader = Cursor::new(player_file_pretty_str.as_bytes());
                ftp_stream.put(&file_name, &mut reader).await.unwrap();

                /*
                so few things for injections.    we only want patreons to be able to inject as adult non survival.  and everyone else can spawn in as non survival juvies but only males
                */
                let guild_id = msg.guild_id.unwrap().0;
                let balance = Unbelievabot::check_balance(guild_id, msg.author.id.0).await.expect("Unable to fetch balance");
                if balance.cash < dino.cost {
                    msg.reply(&ctx, "Not enough cash to buy that dino").await.unwrap();
                    break;
                }
                // let previous_dino = player_object.character_class;
                let user_balance = Unbelievabot::remove_cash(guild_id, msg.author.id.0, dino.cost, 0).await.expect("Unable to remove cash");
                let replace_message = format!("Your {} was replaced with an injected {}", previous_dino, dino.display_name);
                let _ = msg.channel_id.send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.title("Dino injected");
                        e.description(replace_message);
                        e.author(|a| {
                            a.name(&msg.author.name);
                            a.icon_url(msg.author.avatar_url().unwrap());

                            a
                        });
                        e.fields(vec![
                            ("Cash", format!("{}", user_balance.cash), true),
                            ("Bank", format!("{}", user_balance.bank), true),
                        ]);
                        e.colour(Colour::from_rgb(0, 100, 200));
                        e.footer(|f| {
                            f.text(format!("{} Points were withdrawn from your cash", dino.cost));
    
                            f
                        });
    
                        e
                    });
                    // m.reactions(reactions.into_iter());
                    // m.embed(|e| {
                    //     e.title("Dino injected");
                    //     e.description("This is a description");
                    //     e.author(|a| {
                    //         a.name(&msg.author.name);
                    //         a.icon_url(msg.author.avatar_url().unwrap());

                    //         a
                    //     });
                    //     e.colour(Colour::from_rgb(0, 255, 0));
                    //     // e.image("attachment://ferris_eyes.png");
                    //     e.fields(vec![
                    //         ("This is the first field", "This is a field body", true),
                    //         ("This is the second field", "Both of these fields are inline", true),
                    //     ]);
                    //     e.field("This is the third field", "This is not an inline field", false);
                    //     e.footer(|f| {
                    //         f.text("This is a footer");
    
                    //         f
                    //     });
    
                    //     e
                    // });
                    // m.add_file(AttachmentType::Path(Path::new("./ferris_eyes.png")));
                    m
                }).await;
                break;
            }
        }

        if found == false {
            let player_object = Player::new("Anky".to_string(), true);
            let player_file_pretty_str = serde_json::to_string_pretty(&player_object).unwrap();
            let mut reader = Cursor::new(player_file_pretty_str.as_bytes());
            ftp_stream.put(&file_name, &mut reader).await.unwrap();
            msg.reply(&ctx, "Dino injected").await.expect("Unable to reply to message");
        }
    } else {
        msg.reply(&ctx, "Link steam ID first").await.expect("unable to reply to msg");
    }
    Ok(())
}

#[command]
#[aliases("inject", "ij")]
#[only_in("guilds")]
async fn admin_inject(ctx: &Context, msg: &Message) -> CommandResult {
    // let mut data = ctx.data.read().await;
    // {
    //     let ftp_stream = data.get::<FtpStreamContainer>().unwrap();
    // }

    msg.reply(&ctx, "Reading balance").await.expect("Unable to reply to message");
    Ok(())
}

#[command]
#[aliases("randomdino", "rd")]
#[only_in("guilds")]
async fn random_dino(ctx: &Context, msg: &Message) -> CommandResult {
    // let mut data = ctx.data.write();

    msg.reply(&ctx, "Random dino").await.expect("Unable to reply to message");
    Ok(())
}

#[command]
#[aliases("slay", "kill")]
#[only_in("guilds")]
async fn slay_dino(ctx: &Context, msg: &Message) -> CommandResult {
    // let mut data = ctx.data.write();

    msg.reply(&ctx, "Random dino").await.expect("Unable to reply to message");
    Ok(())
}
