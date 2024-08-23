// @generated automatically by Diesel CLI.

diesel::table! {
    areas (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    tributes (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        health -> Int4,
        sanity -> Int4,
        movement -> Int4,
        is_alive -> Bool,
        district -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    areas,
    tributes,
);
