// @generated automatically by Diesel CLI.

diesel::table! {
    area (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
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

diesel::joinable!(tribute -> area (area_id));

diesel::allow_tables_to_appear_in_same_query!(
    area,
    tribute,
);
