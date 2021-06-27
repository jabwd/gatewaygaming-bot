use diesel::prelude::*;

use schema::garage::dsl::*;
use crate::{ schema, schema::garage, DbPoolType };

#[derive(Queryable)]
pub struct Garage {
  pub id: i32,
  pub user_id: i32,
  pub slot_name: String,
  pub character_class: String,
  pub growth: String,
  pub hunger: String,
  pub thirst: String,
  pub stamina: String,
  pub health: String,
  pub bleeding_rate: String,
  pub oxygen: String,
  pub sex: bool,
  pub is_resting: bool,
  pub broken_legs: bool,
  pub progression_points: String,
  pub progression_tier: String,
  pub unlocked_characters: String,
  pub location_thenyaw_island: Option<String>,
  pub rotation_thenyaw_island: Option<String>,
  pub location_isle_v3: Option<String>,
  pub rotation_isle_v3: Option<String>,
  pub camera_rotation_thenyaw_island: Option<String>,
  pub camera_distance_thenyaw_island: Option<String>,
  pub camera_rotation_isle_v3: Option<String>,
  pub camera_distance_isle_v3: Option<String>,
  pub skin_palette_variation: String,
  pub skin_palette_section1: i32,
  pub skin_palette_section2: i32,
  pub skin_palette_section3: i32,
  pub skin_palette_section4: i32,
  pub skin_palette_section5: i32,
  pub skin_palette_section6: i32,
}

#[derive(Insertable)]
#[table_name="garage"]
pub struct GarageSlotInsertable<'a> {
  pub slot_name: &'a String,
  pub character_class: &'a String,
  pub growth: &'a String,
  pub hunger: &'a String,
  pub thirst: &'a String,
  pub stamina: &'a String,
  pub health: &'a String,
  pub bleeding_rate: &'a String,
  pub oxygen: &'a String,
  pub sex: bool,
  pub is_resting: bool,
  pub broken_legs: bool,
  pub progression_points: &'a String,
  pub progression_tier: &'a String,
  pub unlocked_characters: &'a String,
  pub location_thenyaw_island: Option<&'a String>,
  pub rotation_thenyaw_island: Option<&'a String>,
  pub location_isle_v3: Option<&'a String>,
  pub rotation_isle_v3: Option<&'a String>,
  pub camera_rotation_thenyaw_island: Option<&'a String>,
  pub camera_distance_thenyaw_island: Option<&'a String>,
  pub camera_rotation_isle_v3: Option<&'a String>,
  pub camera_distance_isle_v3: Option<&'a String>,
  pub skin_palette_variation: &'a String,
  pub skin_palette_section1: i32,
  pub skin_palette_section2: i32,
  pub skin_palette_section3: i32,
  pub skin_palette_section4: i32,
  pub skin_palette_section5: i32,
  pub skin_palette_section6: i32,
}

impl Garage {
  pub fn slots_for_user(other_user_id: i32, db: &DbPoolType) -> Option<Vec<Garage>> {
    let db = match db.get() {
      Ok(db_instance) => db_instance,
      Err(_) => {
        return None;
      }
    };

    let results = garage.filter(user_id.eq(&other_user_id))
      .load::<Garage>(&db)
      .expect("Unable to load garage slots");

    Some(results)
  }

  pub fn slot_for_id(slot_id: i32, db: &DbPoolType) -> Option<Garage> {
    let db = match db.get() {
      Ok(db_instance) => db_instance,
      Err(_) => {
        return None;
      }
    };

    None
  }

  pub fn update_slot(slot_id: i32, db: &DbPoolType) -> Option<Garage> {
    let db = match db.get() {
      Ok(db_instance) => db_instance,
      Err(_) => {
        return None;
      }
    };

    None
  }

  pub fn save_slot(slot: &GarageSlotInsertable, db: &DbPoolType) -> Option<Garage> {
    let db = match db.get() {
      Ok(db_instance) => db_instance,
      Err(_) => {
        return None;
      }
    };

    let result = diesel::insert_into(garage::table)
      .values(slot)
      .get_result(&db)
      .expect("save slot");

    Some(result)
  }

  pub fn delete_slot(slot_id: i32, db: &DbPoolType) -> Option<usize> {
    let db = match db.get() {
      Ok(db_instance) => db_instance,
      Err(_) => {
        return None;
      }
    };

    let num_deleted = diesel::delete(garage.find(slot_id))
      .execute(&db)
      .expect("Unable to delete slot");

    Some(num_deleted)
  }
}
