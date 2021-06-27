use serenity::{
    prelude::*,
    model::{
        channel::Message,
    },
    framework::standard::{ CommandResult, macros::command },
};

#[command]
#[aliases("bal", "b")]
#[only_in("guilds")]
async fn balance(ctx: &Context, msg: &Message) -> CommandResult {
    let mut data = ctx.data.write();

    msg.reply(&ctx, "Reading balance").await.expect("Unable to reply to message");
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
