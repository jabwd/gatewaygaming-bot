table! {
    balances (id) {
        id -> Int4,
        user_id -> Int4,
        currency_id -> Int4,
        value -> Int8,
    }
}

table! {
    currencies (id) {
        id -> Int4,
        guild_id -> Text,
        name -> Text,
        roles -> Array<Int8>,
        gain_rate -> Int4,
    }
}

table! {
    garage (id) {
        id -> Int4,
        user_id -> Int4,
        slot_name -> Text,
        character_class -> Text,
        growth -> Text,
        hunger -> Text,
        thirst -> Text,
        stamina -> Text,
        health -> Text,
        bleeding_rate -> Text,
        oxygen -> Text,
        sex -> Bool,
        is_resting -> Bool,
        broken_legs -> Bool,
        progression_points -> Text,
        progression_tier -> Text,
        unlocked_characters -> Text,
        location_thenyaw_island -> Nullable<Text>,
        rotation_thenyaw_island -> Nullable<Text>,
        location_isle_v3 -> Nullable<Text>,
        rotation_isle_v3 -> Nullable<Text>,
        camera_rotation_thenyaw_island -> Nullable<Text>,
        camera_distance_thenyaw_island -> Nullable<Text>,
        camera_rotation_isle_v3 -> Nullable<Text>,
        camera_distance_isle_v3 -> Nullable<Text>,
        skin_palette_variation -> Text,
        skin_palette_section1 -> Int4,
        skin_palette_section2 -> Int4,
        skin_palette_section3 -> Int4,
        skin_palette_section4 -> Int4,
        skin_palette_section5 -> Int4,
        skin_palette_section6 -> Int4,
    }
}

table! {
    guilds (id) {
        id -> Int4,
        name -> Text,
        guild_id -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        discord_id -> Text,
        last_active -> Nullable<Timestamptz>,
        steam_id -> Nullable<Text>,
    }
}

joinable!(balances -> currencies (currency_id));
joinable!(balances -> users (user_id));
joinable!(garage -> users (user_id));

allow_tables_to_appear_in_same_query!(
    balances,
    currencies,
    garage,
    guilds,
    users,
);
