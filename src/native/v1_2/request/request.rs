use serde_utils;

use super::placement_type::PlacementTypeID;
use super::layout::LayoutID;
use super::ad_unit::AdUnitID;
use super::context_type::ContextTypeID;
use super::context_sub_type::ContextSubTypeID;
use super::asset::Asset;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Request {
    #[serde(rename = "ver", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(rename = "layout", skip_serializing_if = "Option::is_none")]
    pub layout: Option<LayoutID>,

    #[serde(rename = "adunit", skip_serializing_if = "Option::is_none")]
    pub ad_unit: Option<AdUnitID>,

    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context_type: Option<ContextTypeID>,

    #[serde(rename = "contextsubtype", skip_serializing_if = "Option::is_none")]
    pub context_sub_type: Option<ContextSubTypeID>,

    #[serde(rename = "plcmttype", skip_serializing_if = "Option::is_none")]
    pub placement_type: Option<PlacementTypeID>,

    #[serde(rename = "plcmtcnt", skip_serializing_if = "Option::is_none")]
    pub placement_count: Option<u8>,

    #[serde(rename = "seq", skip_serializing_if = "Option::is_none")]
    pub sequence: Option<u8>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assets: Vec<Asset>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}

impl Request {
    pub fn new() -> Self {
        Default::default()
    }
}
