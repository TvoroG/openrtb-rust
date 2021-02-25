use serde_utils;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Link {
    pub url: String,

    #[serde(rename = "clicktrackers", default, skip_serializing_if = "Vec::is_empty")]
    pub click_trackers: Vec<String>,

    #[serde(rename = "failback", skip_serializing_if = "Option::is_none")]
    pub failback_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}
