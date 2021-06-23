use diesel::prelude::*;
use chrono::prelude::*;

// GuildId
use serenity::model::prelude::{ UserId };
use schema::users::dsl::*;
use crate::{ schema, schema::users, DbPoolType };

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub discord_id: String,
    pub last_active: Option<DateTime<Utc>>,
    pub steam_id: Option<String>,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub discord_id: &'a str,
    pub last_active: &'a DateTime<Utc>,
    pub steam_id: Option<&'a str>,
}

impl User {
    // pub fn get_discord_id(&self) -> UserId {
    //     self.discord_id.parse::<u64>().expect("Could not parse UserId from string").into()
    // }

    pub fn get(discord_user_id: UserId, db: &DbPoolType) -> Self {
        let discord_user_id_str = discord_user_id.to_string();
        let db = db.get().unwrap();
        match users.filter(discord_id.eq(&discord_user_id_str)).first::<User>(&db) {
            Ok(bot_user) => bot_user,
            Err(_) => {
                let new_user = NewUser {
                    discord_id: &discord_user_id_str,
                    last_active: &Utc::now(),
                    steam_id: None,
                };
                let result = diesel::insert_into(users)
                    .values(&new_user)
                    .execute(&db);
                match result {
                    Ok(_) => {
                        users.order(id.desc())
                            .first::<User>(&db)
                            .unwrap()
                    },
                    Err(e) => panic!("{}", e),
                }
            }
        }
    }

    pub fn update_steam_id(user_id: i32, db: &DbPoolType, new_steam_id: &String) -> Self {
        let db = db.get().unwrap();

        diesel::update(users.find(user_id)).set(steam_id.eq(new_steam_id)).get_result::<User>(&db).expect("Unable to find user")
    }
    // pub fn update_last_active(user_id: i32, db: &DbPoolType) -> Self {
    //     let db = db.get().unwrap();

    //     diesel::update(users.find(user_id)).set(last_active.eq(Utc::now())).get_result::<User>(&db).expect("Unable to find user")
    // }
}
