use std::sync::Arc;
use serenity::{
    prelude::*,
    model::{
        channel::Message,
        prelude::{ UserId, Permissions },
    },
    framework::standard::{ Args, CommandResult, macros::command, ArgError::Parse },
};
use async_ftp::FtpStream;
use std::io::Cursor;
use crate::{FtpStreamContainer, entities::player_save::Player, models};

#[command]
#[aliases("cashbuy", "cb")]
#[only_in("guilds")]
async fn cash_buy(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // let mut data = ctx.data.write().await;
    // {
    //     let ftp_stream = data.get::<FtpStreamContainer>().unwrap().clone();

    //     let file_list = ftp_stream.nlst(None).await.unwrap();
    //     println!("Existing files: {:?}", file_list);
    // }

    let steam_id: String = "76561198008239242".to_string();

    // cashbuy rex male
    let dino = match args.single::<String>() {
        Ok(dino_str) => dino_str,
        Err(_) => "".to_string(),
    };

    let gender = match args.single::<String>() {
        Ok(gender_str) => gender_str,
        Err(_) => "".to_string(),
    };

    if dino.len() == 0 {
        msg.reply(&ctx, "Usage: !cb {dino} {gender}").await.expect("Unable to reply to message");
        return Ok(());
    }

    if gender.len() == 0 {
        msg.reply(&ctx, "Usage: !cb {dino} {gender}").await.expect("Unable to reply to message");
        return Ok(());
    }

    // let player = Player::new("Anky".to_string(), true);
    

    let ftp_stream_lock = {
        let data_read = ctx.data.read().await;

        data_read.get::<FtpStreamContainer>().expect("Expected FTP stream").clone()
    };

    {
        let mut ftp_stream = ftp_stream_lock.lock().await;

        let file_list = ftp_stream.nlst(None).await.unwrap();
        let file_name = format!("{}.json", steam_id);

        for file in file_list {
            if file == file_name {
                let mut read_cursor = ftp_stream.simple_retr(&file).await.unwrap();
                let mut player_object: Player = serde_json::from_reader(&mut read_cursor).unwrap();
                player_object.gender = true;
                let player_file_pretty_str = serde_json::to_string_pretty(&player_object).unwrap();
                let mut reader = Cursor::new(player_file_pretty_str.as_bytes());
                ftp_stream.put(&file_name, &mut reader).await.unwrap();
                break;
            }
        }

        
    }

    msg.reply(&ctx, "Dino injected").await.expect("Unable to reply to message");
    Ok(())
}

#[command]
#[aliases("inject", "ij")]
#[only_in("guilds")]
async fn admin_inject(ctx: &Context, msg: &Message) -> CommandResult {
    let mut data = ctx.data.read().await;
    {
        let ftp_stream = data.get::<FtpStreamContainer>().unwrap();
    }

    msg.reply(&ctx, "Reading balance").await.expect("Unable to reply to message");
    Ok(())
}

#[command]
#[aliases("randomdino", "rd")]
#[only_in("guilds")]
async fn random_dino(ctx: &Context, msg: &Message) -> CommandResult {
    let mut data = ctx.data.write();

    msg.reply(&ctx, "Random dino").await.expect("Unable to reply to message");
    Ok(())
}

#[command]
#[aliases("slay", "kill")]
#[only_in("guilds")]
async fn slay_dino(ctx: &Context, msg: &Message) -> CommandResult {
    let mut data = ctx.data.write();

    msg.reply(&ctx, "Random dino").await.expect("Unable to reply to message");
    Ok(())
}
