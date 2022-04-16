use chrono::FixedOffset;
use chrono_tz::Tz;
use serde::{de, Deserialize, Deserializer};

//
#[derive(Deserialize, Debug, Clone)]
pub struct Record {
    #[serde(deserialize_with = "serde_field_with::from_str")]
    pub zone_name: Tz,
    pub country_code: Box<str>,
    pub abbreviation: Box<str>,
    pub time_start: i64,
    #[serde(deserialize_with = "gmt_offset_deserialize")]
    pub gmt_offset: FixedOffset,
    #[serde(deserialize_with = "dst_deserialize")]
    pub dst: bool,
}

fn gmt_offset_deserialize<'de, D>(deserializer: D) -> Result<FixedOffset, D::Error>
where
    D: Deserializer<'de>,
{
    let secs = i32::deserialize(deserializer)?;
    FixedOffset::east_opt(secs).ok_or_else(|| de::Error::custom(format!("Invalid [{}]", secs)))
}

fn dst_deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        n => Err(de::Error::custom(format!("Invalid [{}]", n))),
    }
}
