use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserBalanceResponseEntity {
  pub rank: Option<String>,
  pub user_id: String,
  pub cash: i64,
  pub bank: i64,
  pub total: i64,
}

#[derive(Serialize, Deserialize)]
pub struct UserModifyBalanceEntity {
  pub cash: Option<i64>,
  pub bank: Option<i64>,
  pub reason: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct UserResponseEntity {
  pub user_id: Option<String>,
  pub cash: i64,
  pub bank: i64,
  pub total: i64,
}
