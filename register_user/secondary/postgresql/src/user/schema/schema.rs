// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int8,
        created_at -> Timestamptz,
        modified_at -> Nullable<Timestamptz>,
        #[max_length = 100]
        name -> Varchar,
        email -> Varchar,
        cpf -> Varchar,
    }
}
