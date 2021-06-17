#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;

// use crate::diesel::Connection;
use diesel::{
    PgConnection,
    r2d2::{ ConnectionManager, Pool },
};
use diesel::prelude::*;
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
use async_ftp::FtpStream;
use commands::{
    balance::*,
    dino_injections::*,
};

mod commands;
mod entities;
mod schema;
mod models;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub type DbPoolType = Arc<Pool<ConnectionManager<PgConnection>>>;
pub struct DbPool(DbPoolType);

impl TypeMapKey for DbPool {
    type Value = DbPoolType;
}

pub struct FtpStreamContainer;

impl TypeMapKey for FtpStreamContainer {
    type Value = Arc<FtpStream>;
}

struct Handler;

#[group]
#[commands(
    balance,
    give,
    take,
    cash_buy,
    admin_inject,
    random_dino,
    slay_dino,
)]
struct General;

use serenity::model::guild::MembersIter;
use serenity::futures::StreamExt;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("=> Connected to discord, loading guild data…");

        if let Ok(guilds) = ready.user.guilds(&ctx).await {
            for (index, guild) in guilds.into_iter().enumerate() {
                println!("{}: {}:{}", index, guild.name, guild.id);
                let mut members = MembersIter::<Http>::stream(&ctx, guild.id).boxed();
                while let Some(member_result) = members.next().await {
                    match member_result {
                        Ok(member) => println!(
                            "{} is {}",
                            member,
                            member.display_name()
                        ),
                        Err(error) => eprintln!("Error listing members: {}", error),
                    }
                }
            }
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let data = ctx.data.read().await;
        {
            let db = data.get::<DbPool>().unwrap();
            let user = models::user::User::get(msg.author.id, &db);

            models::user::User::update_last_active(user.id, &db);
        }
        // println!("Message received");

        

        // match msg.member {
        //     Some(member) => println!("User that sent the message: {:?}", member),
        //     None => println!("No use associated with message"),
        // }

        // match msg.guild_id {
        //     Some(guild_id) => println!("Guild id: {:?}", guild_id.0),
        //     None => println!("No guild id found"),
        // }
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    dotenv::dotenv().expect("Failed to load environment variables from .env file");

    // Load in all the env variables we will be using before setting up anything else
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

    println!("FTP Connect?");
    let mut ftp_stream = FtpStream::connect(ftp_address).await
        .expect("Unable to connect to FTP server");
    println!("Stoeff");
    ftp_stream.login(&ftp_username, &ftp_password).await.unwrap();
    println!("Current directory: {}", ftp_stream.pwd().await.unwrap());
    // ftp_stream.cwd("/home/jabwd/TheIsleSaves").await.unwrap();
    // println!("Current directory: {}", ftp_stream.pwd().await.unwrap());
    // let files = ftp_stream.nlst(None).await.unwrap();
    // println!("Files: {:?}", files);

    let ftp_arc = Arc::new(ftp_stream);

    // Set up PSQL connection manager and connection pool
    let manager: ConnectionManager<PgConnection> = ConnectionManager::new(database_url);
    let pool = Pool::builder()
        .max_size(1)
        .build(manager)
        .expect("Could not build database connection pool");
    let pool = Arc::new(pool);

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

    
    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<DbPool>(pool.clone());
        data.insert::<FtpStreamContainer>(ftp_arc);
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register SIGKILL handler");
        shard_manager.lock().await.shutdown_all().await;
        // ftp_arc.as_ref().quit().await;
    });

    if let Err(reason) = client.start().await {
        println!("Client error: {:?}", reason);
    }

    // let _ = ftp_stream.quit().await;
    // println!("Bot shut down");
}
