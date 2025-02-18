// @generated automatically by Diesel CLI.

diesel::table! {
    client (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        modloader -> Text,
        version -> Text,
        mods -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    server (id) {
        id -> Int4,
        name -> Text,
        enabled -> Bool,
        client -> Text,
        online -> Jsonb,
        ip -> Text,
        icon -> Text,
        background -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    client,
    server,
);
