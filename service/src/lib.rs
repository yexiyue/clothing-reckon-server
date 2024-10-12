use serde::{Deserialize, Serialize};

pub mod boss;
pub mod clothing;
pub mod staff;
pub mod user;
mod utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListQueryParams {
    #[serde(
        default = "utils::default_page",
        deserialize_with = "utils::parse_option_u64"
    )]
    pub page: Option<u64>,
    #[serde(
        default = "utils::default_page_size",
        deserialize_with = "utils::parse_option_u64"
    )]
    pub page_size: Option<u64>,

    pub search: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListResult<T>
where
    T: Serialize,
{
    pub total: u64,
    pub data: Vec<T>,
}
