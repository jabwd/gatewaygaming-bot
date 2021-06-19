use crate::models::dino::Dino;
use crate::DbPool;
use serenity::{
    prelude::*,
    model::{
        channel::Message,
    },
    framework::standard::{ Args, CommandResult, macros::command, ArgError::Parse },
};
use async_ftp::FtpStream;
use std::io::Cursor;
use crate::{
    models::user::*,
    FtpStreamContainer,
    entities::player_save::Player,
    internal::*
};

#[command]
#[aliases("reg")]
#[only_in("guilds")]
async fn register(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let steam_id = match args.single::<String>() {
        Ok(steam_id) => steam_id,
        Err(_) => "".to_string(),
    };
    if steam_id.len() != 17 {
        msg.reply(&ctx, "invalid steam id").await.expect("Unable to reply to message");
        return Ok(());
    }

    let data = ctx.data.read().await;
    {
        let db = data.get::<DbPool>().unwrap();
        let user = User::get(msg.author.id, &db);

        User::update_steam_id(user.id, &db, &steam_id);
        msg.delete(&ctx).await.expect("Unable to delete sensitive message content");
        // let channel = match msg.channel_id.to_channel(&ctx).await {
        //     Ok(channel) => channel,
        //     Err(why) => {
        //         println!("Error getting channel {:?}", why);
        //         return Ok(());
        //     },
        // };
        if let Err(why) = msg.channel_id.say(&ctx.http, "Steam ID registered").await {
            println!("Unable to send message to channel {:?}", why);
        }
        
        // msg.reply(&ctx, "steam id saved").await.expect("Unable to reply to message");
    }
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
                let mut read_cursor = ftp_stream.simple_retr(&file).await.unwrap();
                let mut player_object: Player = serde_json::from_reader(&mut read_cursor).unwrap();
                player_object.gender = gender;
                player_object.character_class = dino.character_class.to_string();
                let player_file_pretty_str = serde_json::to_string_pretty(&player_object).unwrap();
                let mut reader = Cursor::new(player_file_pretty_str.as_bytes());
                ftp_stream.put(&file_name, &mut reader).await.unwrap();
                // TODO: List which dino got replaced with what
                // and maybe add a confirmation step
                let reply_msg = format!("Dino {} injected, gender: {}", dino.character_class.to_string(), gender);
                msg.reply(&ctx, reply_msg).await.expect("Unable to reply to message");
                found = true;
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
