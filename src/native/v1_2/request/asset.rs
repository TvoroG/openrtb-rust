use serde_utils;

use super::title::Title;
use super::image::Image;
use super::video::Video;
use super::data::Data;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Asset {
    pub id: u32,

    #[serde(
        default = "serde_utils::default_false",
        skip_serializing_if = "serde_utils::is_false",
        serialize_with = "serde_utils::bool_to_u8",
        deserialize_with = "serde_utils::u8_to_bool"
    )]
    pub required: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Title>,

    #[serde(rename = "img", skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}

impl Asset {
    pub fn new(id: u32, required: bool) -> Self {
        Asset {
            id,
            required,
            title: None,
            image: None,
            video: None,
            data: None,
            ext: None,
        }
    }

    pub fn new_title(id: u32, required: bool, title: Title) -> Self {
        let mut asset = Asset::new(id, required);
        asset.title = Some(title);
        asset
    }

    pub fn new_image(id: u32, required: bool, image: Image) -> Self {
        let mut asset = Asset::new(id, required);
        asset.image = Some(image);
        asset
    }
}
