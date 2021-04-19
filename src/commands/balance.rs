use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

#[command]
#[aliases("bal")]
#[only_in("guilds")]
async fn balance(_ctx: &Context, _msg: &Message) -> CommandResult {
    Ok(())
}

#[command]
#[only_in("guilds")]
async fn give(_ctx: &Context, _msg: &Message) -> CommandResult {
    Ok(())
}
