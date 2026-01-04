// @generated automatically by Diesel CLI.

diesel::table! {
    connection (id) {
        id -> Integer,
        name -> Text,
        password -> Text,
        host -> Text,
    }
}
