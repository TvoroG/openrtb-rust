use serde_utils;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Title {
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}
