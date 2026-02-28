// @generated automatically by Diesel CLI.

diesel::table! {
    connection (id) {
        id -> Integer,
        name -> Text,
        password -> Binary,
        host -> Text,
        port -> Integer,
        username -> Text,
    }
}
