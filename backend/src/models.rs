// use super::schema::publications;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use diesel::{Insertable, IntoSql, Queryable};
use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::sql_types::{Uuid as SqlUuid, Nullable, Text};
use diesel::backend::Backend;
use diesel::result::QueryResult;
use diesel::pg::Pg;


use base64::prelude::*;

use crate::schema::digital_publications;


#[derive(Queryable, Serialize)]
pub struct Source {
    pub uuid: Uuid,
    pub name: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::sources)]
pub struct NewSource {
    pub name: String,
}


#[derive(Queryable, Serialize)]
pub struct DigitalPublication {
    pub uuid: Uuid,
    pub source: Option<Uuid>,

    pub name: String,
    pub description: String,
    pub cover_url: String,
    pub links: Vec<Option<String>>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = digital_publications)]
pub struct NewDigitalPublication {
    pub name: String,
    pub source: Uuid,
    pub description: String,
    pub cover_url: String,
    pub links: Vec<String>,
}