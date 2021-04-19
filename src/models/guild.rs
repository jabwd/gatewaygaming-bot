use diesel::prelude::*;

pub struct Guild {
    pub guild_id: u64,
    pub name: Option<String>,
    pub config: Option<Jsonb>,
}
