use diesel::prelude::*;
use diesel::expression::*;

use serenity::model::prelude::{ UserId, GuildId };

use crate::{ schema::users, DbPoolType };

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub discord_id: u64,
    pub last_active: u64,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub discord_id: &'a u64,
    pub last_active: &'a u64,
}

impl User {
    pub fn get_discord_id(&self) -> UserId {
        return self.discord_id.into()
    }

    pub fn get(discord_user_id: UserId, db: &DbPoolType) -> Self {
        use schema::users::dsl::*;

        let discord_user_id_str = discord_user_id.to_string();
        let db = db.get().unwrap();
        match user.filter(discord_id.eq(&discord_user_id_str)).first::<User>(&db) {
            Ok(botUser) => botUser,
            Err(_) => {
                let newUser = NewUser {
                    discord_id: discord_user_id.into(),
                    last_active: 0,
                };
                let result = diesel::insert_into(crate::schema::user)
                    .values(&newUser)
                    .execute(&db);
                match result {
                    Ok(_) => {
                        users.order(id.desc())
                            .first::<User>(&db)
                            .unwrap()
                    },
                    Err(e) => panic!(e),
                }
            }
        }
    }

    pub fn gen_balance(&self) -> i32 {
        return 0
    }
}
