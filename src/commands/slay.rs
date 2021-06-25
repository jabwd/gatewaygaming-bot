use crate::services::message::MessageResponder;
use crate::services::unbelievabot::*;
use serenity::utils::Colour;
use crate::models::dino::Dino;
use crate::DbPool;
use serenity::{
    prelude::*,
    model::{
        channel::Message,
        guild::GuildContainer,
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
#[aliases("slay", "s", "kill")]
#[only_in("guilds")]
pub async fn slay(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
  Ok(())
}