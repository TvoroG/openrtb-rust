// Copyright (c) 2018 The openrtb-rust authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::geo::Geo;
use serde_utils;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "buyeruid", skip_serializing_if = "Option::is_none")]
    pub buyer_uid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub yob: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,

    #[serde(rename = "customdata", skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<Geo>,

    // #[serde(skip_serializing_if = "Vec::is_empty")]
    // pub data: Vec<Data>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}

impl User {
    pub fn new() -> Self {
        Default::default()
    }
}
