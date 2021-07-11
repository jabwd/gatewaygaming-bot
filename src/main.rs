#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;

use serenity::model::prelude::{Activity, Reaction};
use diesel::{
  PgConnection,
  r2d2::{ ConnectionManager, Pool },
};
use dotenv::dotenv;
use std::{
  collections::HashSet,
  env,
  sync::Arc,
};
use serenity::{
  async_trait,
  client::bridge::gateway::ShardManager,
  framework::{
      StandardFramework,
      standard::macros::group,
  },
  http::Http,
  model::{event::ResumedEvent, gateway::Ready, channel::Message},
  prelude::*
};
use commands::{
  dino_injections::*,
  check_dino::*,
  slay::*,
  teleport::*,
  sex_operation::*,
  dino_garage::*,
};
use services::ftp::FtpConnectionManager;
use contexts::garage::*;
use tokio::sync::RwLock;
use tokio::time::{timeout, Duration};
use tokio::time;
use tokio::task;

mod commands;
mod entities;
mod schema;
mod models;
mod internal;
mod services;
mod contexts;

pub struct ReactionContext {
  pub garage_context_list: Vec<GarageResponseReactionContext>,
  pub bot_user_id: u64,
}

impl TypeMapKey for ReactionContext {
  type Value = Arc<RwLock<ReactionContext>>;
}

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
  type Value = Arc<Mutex<ShardManager>>;
}

pub type FtpPoolType = Arc<bb8::Pool<FtpConnectionManager>>;
pub struct FtpPool(FtpPoolType);

impl TypeMapKey for FtpPool {
  type Value = FtpPoolType;
}

pub type DbPoolType = Arc<Pool<ConnectionManager<PgConnection>>>;
pub struct DbPool(DbPoolType);

impl TypeMapKey for DbPool {
  type Value = DbPoolType;
}

// pub struct FtpStreamContainer;

// impl TypeMapKey for FtpStreamContainer {
//   type Value = Arc<Mutex<FtpStream>>;
// }

struct Handler;

#[group]
#[commands(
  cash_buy,
  admin_inject,
  random_dino,
  slay,
  register,
  dino_request,
  random_dino,
  check_dino,
  teleport,
  sex_change,
  garage_list,
  garage_save_dino,
  garage_delete,
  garage_swap_dino,
)]
struct General;

#[async_trait]
impl EventHandler for Handler {
  async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
    let msg = match add_reaction.message(&ctx).await {
      Ok(msg) => msg,
      Err(why) => {
        println!("Unable to handle reaction: {0}", why);
        return;
      }
    };

    let reaction_context_lock = {
      let data = ctx.data.read().await;
  
      data.get::<crate::ReactionContext>().expect("Unable to find reaction context").clone()
    };
  
    {
      let mut reaction_context = reaction_context_lock.write().await;

      let user = add_reaction.user(&ctx).await.unwrap().id.0;
      if user == reaction_context.bot_user_id {
        return;
      }
      let slice = &reaction_context.garage_context_list[..];
      let mut idx = 0;
      for garage_ctx in slice.iter() {
        if garage_ctx.author_id == user && garage_ctx.msg_id == msg.id.0 {
          let _ = add_reaction.delete(&ctx).await;
          match garage_ctx.response_type {
            GarageResponseType::List => {
              let _ = garage_handle_list_reaction(&ctx, &msg, &add_reaction, &garage_ctx).await;
            },
            GarageResponseType::Delete => {
              let _ = garage_handle_delete_reaction(&ctx, &msg, &add_reaction, &garage_ctx).await;
            },
            GarageResponseType::Swap => {
              let _ = garage_handle_swap_dino(&ctx, &msg, &add_reaction, &garage_ctx).await;
            }
          }
          reaction_context.garage_context_list.remove(idx);
          return;
        }
        idx += 1;
      }
    }
  }

  async fn ready(&self, ctx: Context, ready: Ready) {
    println!("{{GG}} Bot is ready for rulebreaks and general scumbaggery");

    ctx.set_activity(Activity::playing("with ball pythons")).await;

    let reaction_context_lock = {
      let data = ctx.data.read().await;
  
      data.get::<crate::ReactionContext>().expect("Unable to find reaction context").clone()
    };
  
    {
      let mut reaction_context = reaction_context_lock.write().await;
  
      reaction_context.bot_user_id = ready.user.id.0;
    }
  }

  async fn message(&self, _ctx: Context, _msg: Message) {    
  }

  async fn resume(&self, _: Context, _: ResumedEvent) {
  }
}

#[tokio::main]
async fn main() {
  dotenv().ok();
  dotenv::dotenv().expect("Failed to load environment variables from .env file");

  // Load in all the env variables we will be using before setting up anything else
  #[cfg(debug_assertions)]
  let token = env::var("DISCORD_DEV_TOKEN")
    .expect("Expected a discord authentication token in the environment");
  #[cfg(not(debug_assertions))]
  let token = env::var("DISCORD_TOKEN")
    .expect("Expected a discord authentication token in the environment");
  let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");
  let ftp_address = env::var("FTP_ADDRESS")
    .expect("FTP_ADDRESS must be set");
  let ftp_username = env::var("FTP_USERNAME")
    .expect("FTP_USERNAME must be set");
  let ftp_password = env::var("FTP_PASSWORD")
    .expect("FTP_PASSWORD must be set");

  // Set up PSQL connection manager and connection pool
  let manager: ConnectionManager<PgConnection> = ConnectionManager::new(database_url);
  let pool = Pool::builder()
      .max_size(1)
      .build(manager)
      .expect("Could not build database connection pool");
  let pool = Arc::new(pool);

  // Set up FTP connection pool
  let ftp_manager: FtpConnectionManager = FtpConnectionManager::new(&ftp_address, &ftp_username, &ftp_password);
  let ftp_pool = bb8::Pool::builder()
    .max_size(1)
    .build(ftp_manager)
    .await
    .expect("Could not build ftp connection pool");
  let ftp_pool = Arc::new(ftp_pool);

  let http = Http::new_with_token(&token);

  let (owners, _bot_id) = match http.get_current_application_info().await {
      Ok(info) => {
          let mut owners = HashSet::new();
          owners.insert(info.owner.id);
          (owners, info.id)
      },
      Err(reason) => panic!("Could not load discord bot info: {:?}", reason),
  };

  // Create the framework
  #[cfg(debug_assertions)]
  let framework = StandardFramework::new()
      .configure(|c| c
          .owners(owners)
          .prefix("ggdev."))
      .group(&GENERAL_GROUP);

  #[cfg(not(debug_assertions))]
  let framework = StandardFramework::new()
      .configure(|c| c
          .owners(owners)
          .prefix("gg."))
      .group(&GENERAL_GROUP);
  
  let mut client = Client::builder(&token)
          .framework(framework)
          .event_handler(Handler)
          .await
          .expect("Err creating client");

  let reaction_context = ReactionContext
  {
    garage_context_list: Vec::new(),
    bot_user_id: 0,
  };
  let reaction_context = Arc::new(RwLock::new(reaction_context));
  
  {
    let mut data = client.data.write().await;
    data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    data.insert::<DbPool>(pool.clone());
    data.insert::<FtpPool>(ftp_pool.clone());
    data.insert::<ReactionContext>(reaction_context.clone());
  }

  let shard_manager = client.shard_manager.clone();

  println!("=> Starting discord service");
  let forever = task::spawn(async move {
    let pool = &ftp_pool;
    let mut interval = time::interval(Duration::from_secs(30));
    loop {
      interval.tick().await;
      println!("=> Check ftp connection");
      let mut ftp_stream = pool.get().await.expect("Expected FTP connection");
      ftp_stream.noop().await.expect("No op failed on FTP Connection");
    }
  });
  tokio::spawn(async move {
    forever.await.expect("Loop not handled");
    tokio::signal::ctrl_c().await.expect("Could not register SIGKILL handler");
    shard_manager.lock().await.shutdown_all().await;
  });

  if let Err(reason) = client.start().await {
    println!("Client error: {:?}", reason);
  }
}
