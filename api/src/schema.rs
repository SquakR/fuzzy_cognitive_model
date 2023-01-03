// @generated automatically by Diesel CLI.

diesel::table! {
    sessions (id) {
        id -> Int4,
        is_active -> Bool,
        user_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        first_name -> Varchar,
        second_name -> Nullable<Varchar>,
        last_name -> Varchar,
        avatar -> Nullable<Varchar>,
    }
}

diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    sessions,
    users,
);
