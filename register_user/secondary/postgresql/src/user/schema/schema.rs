// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        modified_at -> Nullable<Timestamptz>,
        #[max_length = 100]
        name -> Varchar,
        email -> Varchar,
    }
}
