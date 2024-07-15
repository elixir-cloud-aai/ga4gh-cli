/*
 * GA4GH service-info API specification
 *
 * A way for a service to describe basic metadata concerning a service alongside a set of capabilities and/or limitations of the service. More information on [GitHub](https://github.com/ga4gh-discovery/ga4gh-service-info/).
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: ga4gh-discovery-networks@ga4gh.org
 * Generated by: https://openapi-generator.tech
 */

#![allow(unused_imports)]
#![allow(clippy::empty_docs)]
use crate::serviceinfo::models;
use serde::{Deserialize, Serialize};

/// Service : GA4GH service
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Service {
    /// Unique ID of this service. Reverse domain name notation is recommended, though not required. The identifier should attempt to be globally unique so it can be used in downstream aggregator services e.g. Service Registry.
    #[serde(rename = "id")]
    pub id: String,
    /// Name of this service. Should be human readable.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: Box<models::ServiceType>,
    /// Description of the service. Should be human readable and provide information about the service.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "organization")]
    pub organization: Box<models::ServiceOrganization>,
    /// URL of the contact for the provider of this service, e.g. a link to a contact form (RFC 3986 format), or an email (RFC 2368 format).
    #[serde(rename = "contactUrl", skip_serializing_if = "Option::is_none")]
    pub contact_url: Option<String>,
    /// URL of the documentation of this service (RFC 3986 format). This should help someone learn how to use your service, including any specifics required to access data, e.g. authentication.
    #[serde(rename = "documentationUrl", skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,
    /// Timestamp describing when the service was first deployed and available (RFC 3339 format)
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Timestamp describing when the service was last updated (RFC 3339 format)
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Environment the service is running in. Use this to distinguish between production, development and testing/staging deployments. Suggested values are prod, test, dev, staging. However this is advised and not enforced.
    #[serde(rename = "environment", skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    /// Version of the service being described. Semantic versioning is recommended, but other identifiers, such as dates or commit hashes, are also allowed. The version should be changed whenever the service is updated.
    #[serde(rename = "version")]
    pub version: String,
}

impl Service {
    /// GA4GH service
    pub fn new(id: String, name: String, r#type: models::ServiceType, organization: models::ServiceOrganization, version: String) -> Service {
        Service {
            id,
            name,
            r#type: Box::new(r#type),
            description: None,
            organization: Box::new(organization),
            contact_url: None,
            documentation_url: None,
            created_at: None,
            updated_at: None,
            environment: None,
            version,
        }
    }
}

