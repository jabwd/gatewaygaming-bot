use serenity::{
    prelude::*,
    model::{
        channel::Message,
        prelude::{ UserId, Permissions },
    },
    framework::standard::{ Args, CommandResult, macros::command, ArgError::Parse },
};
use async_ftp::FtpStream;
use crate::{FtpStreamContainer, entities::player_save::Player, models};

#[command]
#[aliases("cashbuy", "cb")]
#[only_in("guilds")]
async fn cash_buy(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut data = ctx.data.write();

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

    let player = Player::new("Anky".to_string(), true);
    let str = player.json_string();

    msg.reply(&ctx, str).await.expect("Unable to reply to message");
    Ok(())
}

#[command]
#[aliases("inject", "ij")]
#[only_in("guilds")]
async fn admin_inject(ctx: &Context, msg: &Message) -> CommandResult {
    let mut data = ctx.data.write();

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
