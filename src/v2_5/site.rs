// Copyright (c) 2018 The openrtb-rust authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::category::Category;
use super::publisher::Publisher;
use serde_utils;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Site {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cat: Vec<Category>,

    #[serde(rename = "sectioncat", default, skip_serializing_if = "Vec::is_empty")]
    pub section_cat: Vec<Category>,

    #[serde(rename = "pagecat", default, skip_serializing_if = "Vec::is_empty")]
    pub page_cat: Vec<Category>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    #[serde(rename="ref", skip_serializing_if = "Option::is_none")]
    pub referrer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "serde_utils::mbool_to_u8",
        deserialize_with = "serde_utils::u8_to_mbool"
    )]
    pub mobile: Option<bool>,

    #[serde(
        rename = "privacypolicy",
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "serde_utils::mbool_to_u8",
        deserialize_with = "serde_utils::u8_to_mbool"
    )]
    pub privacy_policy: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<Publisher>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}

impl Site {
    pub fn new(id: String) -> Self {
        Site {
            id,
            name: None,
            domain: None,
            cat: vec![],
            section_cat: vec![],
            page_cat: vec![],
            page: None,
            referrer: None,
            search: None,
            mobile: None,
            privacy_policy: None,
            publisher: None,
            keywords: None,
            ext: None,
        }
    }
}
