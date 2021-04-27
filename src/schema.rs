table! {
    balances (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        currency_id -> Nullable<Int4>,
        value -> Nullable<Int8>,
    }
}

table! {
    currencies (id) {
        id -> Int4,
        guild_id -> Nullable<Int8>,
        name -> Nullable<Text>,
        roles -> Nullable<Array<Int8>>,
        gain_rate -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Int4,
        discord_id -> Nullable<Int8>,
        last_active -> Nullable<Timestamptz>,
    }
}

joinable!(balances -> currencies (currency_id));
joinable!(balances -> users (user_id));

allow_tables_to_appear_in_same_query!(
    balances,
    currencies,
    users,
);
