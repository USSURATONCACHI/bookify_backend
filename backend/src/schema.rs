// @generated automatically by Diesel CLI.

diesel::table! {
    digital_publications (uuid) {
        uuid -> Uuid,
        source -> Nullable<Uuid>,
        name -> Varchar,
        description -> Varchar,
        cover_url -> Varchar,
        links -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    sources (uuid) {
        uuid -> Uuid,
        name -> Varchar,
    }
}

diesel::joinable!(digital_publications -> sources (source));

diesel::allow_tables_to_appear_in_same_query!(
    digital_publications,
    sources,
);
