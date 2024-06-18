// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        account -> Text,
        password -> Text,
        identity -> Bool,
    }
}

diesel::table! {
    users_account (account_id, user_id) {
        account_id -> Integer,
        user_id -> Integer,
    }
}

diesel::table! {
    website_account (id) {
        id -> Nullable<Integer>,
        account -> Text,
        password -> Text,
        site_name -> Text,
        site_url -> Text,
        note -> Nullable<Text>,
    }
}

diesel::joinable!(users_account -> users (user_id));
diesel::joinable!(users_account -> website_account (account_id));

diesel::allow_tables_to_appear_in_same_query!(
    users,
    users_account,
    website_account,
);
