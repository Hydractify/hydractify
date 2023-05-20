// @generated automatically by Diesel CLI.

diesel::table! {
    self_role (id) {
        id -> Int8,
        style -> Int4,
        emoji -> Nullable<Text>,
    }
}
