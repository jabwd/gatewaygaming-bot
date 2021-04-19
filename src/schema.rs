table! {
    guild (id) {
        id -> Int8,
        name -> Nullable<Text>,
        config -> Nullable<Jsonb>,
    }
}

table! {
    user (id) {
        id -> Int4,
        guild_id -> Nullable<Int8>,
        discord_id -> Nullable<Int8>,
        name -> Nullable<Text>,
        balance -> Nullable<Int8>,
        last_active -> Nullable<Timestamptz>,
    }
}

allow_tables_to_appear_in_same_query!(
    guild,
    user,
);
