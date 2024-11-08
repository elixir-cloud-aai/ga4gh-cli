/*
 * Data Repository Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.4.0
 * Contact: ga4gh-cloud@ga4gh.org
 * Generated by: https://openapi-generator.tech
 */

#![allow(unused_imports)]
#![allow(clippy::empty_docs)]
use crate::clients::drs::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostAccessUrlRequest {
    /// the encoded JWT GA4GH Passport that contains embedded Visas.  The overall JWT is signed as are the individual Passport Visas.
    #[serde(rename = "passports", skip_serializing_if = "Option::is_none")]
    pub passports: Option<Vec<String>>,
}

impl PostAccessUrlRequest {
    pub fn new() -> PostAccessUrlRequest {
        PostAccessUrlRequest {
            passports: None,
        }
    }
}

