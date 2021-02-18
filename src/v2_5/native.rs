// Copyright (c) 2018 The openrtb-rust authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde_utils;

use super::api_framework::APIFramework;
use super::creative_attribute::CreativeAttribute;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Native {
    pub request: String,

    #[serde(rename = "ver", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub api: Vec<APIFramework>,

    #[serde(rename = "battr", skip_serializing_if = "Vec::is_empty")]
    pub blocked_attrs: Vec<CreativeAttribute>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}

impl Native {
    pub fn new(request: String) -> Self {
        Native {
            request,
            version: None,
            api: vec![],
            blocked_attrs: vec![],
            ext: None,
        }
    }
}
