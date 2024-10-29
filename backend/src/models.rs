use super::schema::publications;
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use base64::prelude::*;

#[derive(Queryable, Serialize)]
pub struct Publication {
    pub id: Uuid,
    pub name: String,

    pub filename: String,
    #[serde(serialize_with = "serialize_base64", deserialize_with = "deserialize_base64")]
    pub file: Vec<u8>,
}

#[derive(Queryable, Serialize)]
pub struct PublicationName {
    pub id: Uuid,
    pub name: String,
    pub filename: String,
}


#[derive(Insertable, Deserialize)]
#[diesel(table_name = publications)]
pub struct NewPublication {
    pub name: String,
    pub filename: String,

    #[serde(serialize_with = "serialize_base64", deserialize_with = "deserialize_base64")]
    pub file: Vec<u8>,
}

fn serialize_base64<S>(data: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let encoded = BASE64_STANDARD.encode(data);
    serializer.serialize_str(&encoded)
}

// Deserialization function for Base64
fn deserialize_base64<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let base64_str: String = Deserialize::deserialize(deserializer)?;
    BASE64_STANDARD.decode(&base64_str).map_err(serde::de::Error::custom)
}


// pub struct NewPublicationJson {
//     pub name: String,
//     pub file: String,
// }