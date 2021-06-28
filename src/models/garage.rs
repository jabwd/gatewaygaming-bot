use diesel::prelude::*;

use schema::garage::dsl::*;
use crate::{DbPoolType, entities::player::Player, schema, schema::garage};

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
  pub user_id: i32,
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

impl GarageSlotInsertable<'_> {
  pub fn from_player_object<'a>(player_object: &'a Player, author_id: i32, new_slot_name: &'a String) -> GarageSlotInsertable<'a> {
    let slot = GarageSlotInsertable
    {
      user_id: author_id,
      slot_name: new_slot_name,
      character_class: &player_object.character_class,
      growth: &player_object.growth,
      hunger: &player_object.hunger,
      thirst: &player_object.thirst,
      stamina: &player_object.stamina,
      health: &player_object.health,
      bleeding_rate: &player_object.bleeding_rate,
      oxygen: &player_object.oxygen,
      sex: player_object.gender,
      is_resting: player_object.is_resting,
      broken_legs: player_object.broken_legs,
      progression_points: &player_object.progression_points,
      progression_tier: &player_object.progression_tier,
      unlocked_characters: &player_object.unlocked_characters,
      location_thenyaw_island: None,
      rotation_thenyaw_island: None,
      location_isle_v3: None,
      rotation_isle_v3: None,
      camera_rotation_thenyaw_island: None,
      camera_distance_thenyaw_island: None,
      camera_rotation_isle_v3: None,
      camera_distance_isle_v3: None,
      skin_palette_variation: &player_object.skin_palette_variation,
      skin_palette_section1: player_object.skin_palette_section1,
      skin_palette_section2: player_object.skin_palette_section2,
      skin_palette_section3: player_object.skin_palette_section3,
      skin_palette_section4: player_object.skin_palette_section4,
      skin_palette_section5: player_object.skin_palette_section5,
      skin_palette_section6: player_object.skin_palette_section6,
    };

    slot
  }
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
