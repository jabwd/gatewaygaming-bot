use crate::models::garage::*;
use chrono::prelude::*;

#[derive(Debug)]
pub enum GarageResponseType {
  List,
  Delete,
  Swap
}

#[derive(Debug)]
pub struct GarageResponseReactionContext {
  pub msg_id: u64,
  pub author_id: u64,
  pub slots: Vec<Garage>,
  pub response_type: GarageResponseType,
  pub created: DateTime<Utc>,
}
