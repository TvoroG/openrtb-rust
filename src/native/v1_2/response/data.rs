use serde_utils;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Data {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    pub value: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}
