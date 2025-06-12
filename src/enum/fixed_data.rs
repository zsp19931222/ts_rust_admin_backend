use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum FixedDataConfig {
    #[serde(rename = "STRENGTH_NUMBERS")]
    StrengthNumbers,
    #[serde(rename = "INTRODUCTION")]
    Introduction,
}

impl FixedDataConfig {
    pub fn get_name(&self) -> &'static str {
        match self {
            FixedDataConfig::StrengthNumbers => "服务数",
            FixedDataConfig::Introduction => "简介",
        }
    }
}