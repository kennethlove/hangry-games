// @generated automatically by Diesel CLI.

diesel::table! {
    action (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Text,
    }
}

diesel::table! {
    area (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    game (id) {
        id -> Int4,
        name -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    tribute (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        health -> Int4,
        sanity -> Int4,
        movement -> Int4,
        is_alive -> Bool,
        district -> Int4,
        area_id -> Nullable<Int4>,
    }
}

diesel::table! {
    tribute_action (id) {
        id -> Int4,
        tribute_id -> Int4,
        action_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::joinable!(tribute -> area (area_id));
diesel::joinable!(tribute_action -> action (action_id));
diesel::joinable!(tribute_action -> tribute (tribute_id));

diesel::allow_tables_to_appear_in_same_query!(
    action,
    area,
    game,
    tribute,
    tribute_action,
);
