use serde_utils;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Title {
    #[serde(rename = "len")]
    pub length: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}

impl Title {
    pub fn new(length: u32) -> Self {
        Title {
            length,
            ext: None,
        }
    }
}
