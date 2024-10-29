// @generated automatically by Diesel CLI.

diesel::table! {
    publications (id) {
        id -> Uuid,
        name -> Varchar,
        file -> Bytea,
    }
}
