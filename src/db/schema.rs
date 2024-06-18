// @generated automatically by Diesel CLI.

diesel::table! {
    website_account (id) {
        id -> Nullable<Integer>,
        account -> Text,
        password -> Text,
        site_url -> Text,
        site_name -> Nullable<Text>,
        note -> Nullable<Text>,
    }
}
