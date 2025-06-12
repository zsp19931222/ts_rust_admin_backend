use serde::{Deserialize, Serialize};
use crate::enums::fixed_data::FixedDataConfig;

#[derive(Debug, Serialize)]
pub struct FixedDataResponse {
    pub config: FixedDataConfig,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct FixedDataConfigEntity {
    pub key: String,
    pub value: Option<String>,
    pub r#type: String,
}

#[derive(Debug, Serialize)]
pub struct FixedDataConfigResponse {
    pub key: String,
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FixedDataConfigUpdateRequest {
    pub r#type: FixedDataConfig,
    pub list: Vec<FixedDataConfigItem>,
}

#[derive(Debug, Deserialize)]
pub struct FixedDataConfigItem {
    pub key: String,
    pub value: String,
} 