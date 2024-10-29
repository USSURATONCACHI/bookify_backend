use super::schema::publications;
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Queryable, Serialize)]
pub struct Publication {
    pub id: Uuid,
    pub name: String,
    pub file: Vec<u8>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = publications)]
pub struct NewPublication {
    pub name: String,
    pub file: Vec<u8>,
}