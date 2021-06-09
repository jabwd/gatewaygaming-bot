use diesel::prelude::*;
use crate::{ schema, schema::currencies, DbPoolType };
use schema::currencies::dsl::*;

#[derive(Queryable)]
pub struct Currency {
    pub id: u64,
    pub guild_id: u64,
    pub name: String,
    pub roles: Vec<u64>,
    pub gain_rate: u64,
}

#[derive(Insertable)]
#[table_name = "currencies"]
struct NewCurrency<'a> {
    pub name: &'a str,
    pub roles: &'a Vec<u64>,
}

impl Currency {
    pub fn list(db: &DbPoolType) {
        
    }
}
