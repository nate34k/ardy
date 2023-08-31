use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
pub struct Hello {
    pub hello: String,
}

fn deserialize_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M")
        .map_err(serde::de::Error::custom)
}

#[derive(Deserialize)]
pub struct ItemData {
    pub item_name: String,
    pub quantity: i64,
    pub total_price: i64,
    pub is_purchase: bool,
    #[serde(deserialize_with = "deserialize_datetime")]
    pub timestamp: NaiveDateTime,
}