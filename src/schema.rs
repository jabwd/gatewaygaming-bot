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

allow_tables_to_appear_in_same_query!(
    balances,
    currencies,
    guilds,
    users,
);
