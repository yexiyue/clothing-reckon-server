use serde::{Deserialize, Deserializer};

pub fn default_page_size() -> Option<u64> {
    Some(10)
}

pub fn default_page() -> Option<u64> {
    Some(0)
}

pub fn parse_option_u64<'de, D>(deserializer: D) -> std::result::Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;

    match s {
        Some(s) => Ok(s.parse::<u64>().ok()),
        None => Ok(None),
    }
}

pub fn parse_svc<'de, D>(deserializer: D) -> std::result::Result<Option<Vec<i32>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => Ok(Some(
            s.split(',').filter_map(|s| s.parse::<i32>().ok()).collect(),
        )),
        None => Ok(None),
    }
}