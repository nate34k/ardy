use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Deserialize)]
pub struct Hello {
    pub hello: String,
}

fn deserialize_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M").map_err(serde::de::Error::custom)
}

fn serialize_datetime<S>(datetime: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let s = format!("{}", datetime.format("%Y-%m-%dT%H:%M"));
    serializer.serialize_str(&s)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemData {
    pub id: i64,
    pub item_name: String,
    pub quantity: i64,
    pub total_price: i64,
    pub is_purchase: bool,
    #[serde(deserialize_with = "deserialize_datetime")]
    #[serde(serialize_with = "serialize_datetime")]
    pub timestamp: NaiveDateTime,
}
