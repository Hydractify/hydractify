// @generated automatically by Diesel CLI.

diesel::table! {
    self_role (id) {
        id -> Int8,
        style -> Int4,
        emoji -> Nullable<Text>,
    }
}

diesel::table! {
    starboard (message_id) {
        message_id -> Int8,
        starboard_id -> Nullable<Int8>,
        stars -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(self_role, starboard,);
