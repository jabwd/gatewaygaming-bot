use serenity::{
    prelude::*,
    model::{
        channel::Message,
        prelude::{ UserId, Permissions },
    },
    framework::standard::{ Args, CommandResult, macros::command, ArgError::Parse },
};

use crate::{ DbPool, models };

#[command]
#[aliases("bal", "b")]
#[only_in("guilds")]
async fn balance(ctx: &Context, msg: &Message) -> CommandResult {
    let mut data = ctx.data.write();

    msg.reply(&ctx, "Reading balance");
    Ok(())
}

#[command]
#[aliases("g")]
#[only_in("guilds")]
async fn give(_ctx: &Context, _msg: &Message) -> CommandResult {
    Ok(())
}

#[command]
#[aliases("t")]
#[only_in("guilds")]
async fn take(_ctx: &Context, _msg: &Message) -> CommandResult {
    Ok(())
}
