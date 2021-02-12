use serde_utils;

use super::image_type::ImageTypeID;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Image {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_id: Option<ImageTypeID>,

    #[serde(rename = "w", skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    #[serde(rename = "wmin", skip_serializing_if = "Option::is_none")]
    pub width_min: Option<u32>,

    #[serde(rename = "h", skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    #[serde(rename = "hmin", skip_serializing_if = "Option::is_none")]
    pub height_min: Option<u32>,

    #[serde(rename = "mimes", skip_serializing_if = "Vec::is_empty")]
    pub mimes: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}
