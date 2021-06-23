use crate::entities::unbelievabot::*;
use std::env;

pub struct Unbelievabot;

const BASE_URL: &str = "https://unbelievaboat.com/api/v1";

impl Unbelievabot {
  pub async fn check_balance(guild_id: u64, user_id: u64) -> Result<UserBalanceResponseEntity, reqwest::Error> {
    let key = env::var("UNBELIEVABOT_TOKEN").expect("Expected a unbelievabot api key in this environment");
    let url = format!("{}/guilds/{}/users/{}", BASE_URL, guild_id, user_id);
    let client = reqwest::Client::new();
    let res = client.get(&url)
      .header("Authorization", key)
      .send()
      .await?
      .json::<UserBalanceResponseEntity>().await?;

    Ok(res)
  }

  pub async fn remove_cash(guild_id: u64, user_id: u64, remove_cash: i64, remove_bank: i64) -> Result<UserResponseEntity, reqwest::Error> {
    let key = env::var("UNBELIEVABOT_TOKEN").expect("Expected a unbelievabot api key in this environment");
    let url = format!("{}/guilds/{}/users/{}", BASE_URL, guild_id, user_id);
    let modify_request = UserModifyBalanceEntity
    {
      cash: Some(-1 * remove_cash),
      bank: Some(-1 * remove_bank),
      reason: Some("Dinobot action".to_string()),
    };

    let client = reqwest::Client::new();
    let res = client.patch(&url)
      .header("Authorization", key)
      .json(&modify_request)
      .send()
      .await?
      .json::<UserResponseEntity>().await?;

    Ok(res)
  }
}
