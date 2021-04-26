use diesel::prelude::*;

struct Currency {
    pub id: u64,
    pub guild_id: u64,
    pub name: String,
    pub roles: Vec<u64>,
    pub gain_rate: u64,
}
