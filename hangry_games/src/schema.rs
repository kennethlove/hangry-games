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
        day -> Nullable<Int4>,
        closed_areas -> Nullable<Array<Nullable<Int4>>>,
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
        district -> Int4,
        area_id -> Nullable<Int4>,
        game_id -> Nullable<Int4>,
        day_killed -> Nullable<Int4>,
        kills -> Nullable<Int4>,
        wins -> Nullable<Int4>,
        defeats -> Nullable<Int4>,
        draws -> Nullable<Int4>,
        games -> Nullable<Int4>,
        bravery -> Nullable<Int4>,
        loyalty -> Nullable<Int4>,
        speed -> Nullable<Int4>,
        intelligence -> Nullable<Int4>,
        persuasion -> Nullable<Int4>,
        luck -> Nullable<Int4>,
        #[max_length = 255]
        killed_by -> Nullable<Varchar>,
        strength -> Nullable<Int4>,
        defense -> Nullable<Int4>,
        is_hidden -> Nullable<Bool>,
        dexterity -> Nullable<Int4>,
        #[max_length = 255]
        status -> Varchar,
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
diesel::joinable!(tribute -> game (game_id));
diesel::joinable!(tribute_action -> action (action_id));
diesel::joinable!(tribute_action -> tribute (tribute_id));

diesel::allow_tables_to_appear_in_same_query!(
    action,
    area,
    game,
    tribute,
    tribute_action,
);
