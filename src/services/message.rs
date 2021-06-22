use serenity::{
  prelude::*,
  model::{
      channel::Message,
  },
  framework::standard::{ Args, CommandResult, macros::command },
};

pub struct MessageResponder<'a> {
  pub ctx: &'a Context,
  pub msg: &'a Message,
}

// impl MessageResponder<'a> {

// }
