// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
        author -> Varchar,
        published -> Bool,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Text,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        email -> Nullable<Text>,
        phone -> Nullable<Text>,
        active -> Nullable<Bool>,
        balance -> Nullable<Text>,
        string_rep -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    books,
    posts,
    users,
);
