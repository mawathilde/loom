// @generated automatically by Diesel CLI.

diesel::table! {
    users (uuid) {
        uuid -> Text,

        created_at -> Timestamp,

        updated_at -> Timestamp,

        email -> Text,

        auth_key_hash -> Binary,

        salt -> Binary,
    }
}
