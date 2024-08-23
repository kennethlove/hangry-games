// @generated automatically by Diesel CLI.

diesel::table! {
    areas (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}
