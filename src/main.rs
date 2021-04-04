extern crate dotenv;

mod commands;

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("=> Starting Gateway bot");
    println!("Hello, world!");

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a discord token defined in the environment");
    
    let framework = StandardFramework::new()
        .configure(|c| c
            .prefix("g/"));
}
