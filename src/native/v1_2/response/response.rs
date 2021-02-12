use serde_utils;

use super::asset::Asset;
use super::link::Link;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Response {
    #[serde(rename = "ver", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub assets: Vec<Asset>,

    pub link: Link,

    #[serde(rename = "imptrackers", skip_serializing_if = "Vec::is_empty")]
    pub imp_trackers: Vec<String>,

    #[serde(rename = "jstracker", skip_serializing_if = "Option::is_none")]
    pub js_tracker: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}
