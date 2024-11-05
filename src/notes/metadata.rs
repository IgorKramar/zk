use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Local, TimeZone};
use serde::{Deserializer, Serializer};

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteMetadata {
    pub title: String,
    #[serde(serialize_with = "serialize_datetime", deserialize_with = "deserialize_datetime")]
    pub created: DateTime<Utc>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub links: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl NoteMetadata {
    pub fn new(title: String) -> Self {
        NoteMetadata {
            title,
            created: Utc::now(),
            tags: Vec::new(),
            links: Vec::new(),
            description: None,
        }
    }
}

fn serialize_datetime<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let local_time = date.with_timezone(&Local);
    serializer.serialize_str(&local_time.format("%Y-%m-%d %H:%M:%S %z").to_string())
}

fn deserialize_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let time_str = String::deserialize(deserializer)?;
    
    // Пробуем разные форматы даты
    let formats = [
        "%Y-%m-%d %H:%M:%S %z",
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%dT%H:%M:%S%z",
        "%Y-%m-%dT%H:%M:%SZ",
    ];

    for format in formats {
        if let Ok(dt) = Local.datetime_from_str(&time_str, format) {
            return Ok(dt.with_timezone(&Utc));
        }
    }

    Err(serde::de::Error::custom(format!(
        "Не удалось распарсить дату: {}",
        time_str
    )))
} 