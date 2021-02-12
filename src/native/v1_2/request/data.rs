use serde_utils;

use super::data_type::DataTypeID;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Data {
    #[serde(rename = "type")]
    pub type_id: DataTypeID,

    #[serde(rename = "len")]
    pub length: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}
