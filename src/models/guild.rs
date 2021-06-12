use diesel::prelude::*;
use serenity::model::prelude::{ GuildId };
use crate::{ schema, schema::guilds, DbPoolType };
use schema::guilds::dsl::*;

#[derive(Queryable)]
pub struct Guild {
    pub id: i32,
    pub name: String,
    pub guild_id: String,
}

#[derive(Insertable)]
#[table_name = "guilds"]
pub struct NewGuild<'a> {
    pub name: &'a str,
    pub guild_id: &'a str,
}

impl Guild {
    pub fn get_discord_id(&self) -> GuildId {
        self.guild_id.parse::<u64>().expect("Could not parse UserId from string").into()
    }

    pub fn get(other_guild_id: GuildId, other_name: String, db: &DbPoolType) -> Self {
        let guild_id_str = other_guild_id.to_string();
        let db = db.get().unwrap();
        match guilds.filter(guild_id.eq(&guild_id_str)).first::<Guild>(&db) {
            Ok(bot_guild) => bot_guild,
            Err(_) => {
                let new_guild = NewGuild {
                    name: &other_name,
                    guild_id: &guild_id_str,
                };
                let result = diesel::insert_into(guilds)
                    .values(&new_guild)
                    .execute(&db);
                match result {
                    Ok(_) => {
                        guilds.order(id.desc())
                            .first::<Guild>(&db)
                            .unwrap()
                    },
                    Err(e) => panic!("{}", e),
                }
            }
        }
    }
}
