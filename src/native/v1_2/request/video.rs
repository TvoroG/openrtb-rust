use serde_utils;

use crate::v2_5::Protocol;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Video {
    #[serde(rename = "mimes", default, skip_serializing_if = "Vec::is_empty")]
    pub mimes: Vec<String>,

    #[serde(rename = "minduration", skip_serializing_if = "Option::is_none")]
    pub min_duration: Option<u32>,

    #[serde(rename = "maxduration", skip_serializing_if = "Option::is_none")]
    pub max_duration: Option<u32>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub protocols: Vec<Protocol>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}
