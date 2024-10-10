use serde::{Deserialize, Serialize};

pub mod boss;
pub mod user;

#[derive(Debug, Serialize, Deserialize)]
pub struct ListQueryParams {
    pub page: Option<u64>,
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
